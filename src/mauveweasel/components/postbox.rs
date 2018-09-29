use std::path::PathBuf;
use std::result::Result;
use std::io;
use std::fs;
use std::fs::File;
use mauveweasel::cookie;
use mauveweasel::cookie::Cookie;
use mauveweasel::http::{Request,Response};
use mauveweasel::options::Config;
use uuid::Uuid;
use serde_urlencoded;
use mauveweasel::server::DynamicContentServer;
use bincode;
use chrono::serde::ts_seconds;
use chrono::{Duration, DateTime, Utc};

pub enum PostboxError {
    MissingField( Vec< String >, PostboxMessage ),
    BadPath,
    BadParse
}

pub struct Postbox {
    path: PathBuf,
    message: PostboxMessage
}

#[derive(Clone,Serialize,Deserialize)]
pub struct PostboxMessage {
    pub name: String,
    pub comment: String,
    pub email: String
}

#[derive(Serialize,Deserialize)]
pub struct ValidationCookie {
   id: String,
   #[serde(with = "ts_seconds")]
   creation: DateTime< Utc >,
   pub validation_errors: Vec< String >,
   pub contents: PostboxMessage
}

fn get_default_validation_cookie() -> ValidationCookie {
    ValidationCookie{
        id: format!( "{}", Uuid::new_v4() ),
        creation: Utc::now(),
        validation_errors: Vec::new(),
        contents: PostboxMessage{
            name: String::new(),
            comment: String::new(),
            email: String::new()
        }
    }
}

fn get_default_postbox_message() -> PostboxMessage {
    PostboxMessage {
        name: String::new(),
        comment: String::new(),
        email: String::new()
    }
}

impl ValidationCookie {
    pub fn from_request( request: &Request, server: &DynamicContentServer ) -> ValidationCookie {
        match request.raw_headers().get( "cookie" ) {
            Some( cookie_string ) => match cookie::parse( &cookie_string ).get( "postbox_validation" ) {
                Some( uuid_string ) => match File::open( server.config().cookiejar_directory().to_string() + &format!( "/session/{}.bck", uuid_string ) ) {
                    Ok( file ) => bincode::deserialize_from( file ).unwrap_or( get_default_validation_cookie() ),
                    Err( _ ) => get_default_validation_cookie()
                },
                None => get_default_validation_cookie()
            },
            None => get_default_validation_cookie()
        }
    }
}

impl Cookie for ValidationCookie {
   fn name( &self ) -> &str {
       "postbox_validation"
   }

   fn value( &self ) -> &str {
       &self.id
   }

   fn max_age( &self ) -> Option< Duration > {
       Some( Duration::days( 1 ) )
   }

   fn get_expiry( &self ) -> Option< DateTime< Utc > > {
       Some( self.creation + self.max_age().unwrap() )
   }

   fn save( &self, config: &Config ) -> io::Result< () > {
       let file = File::create( config.cookiejar_directory().to_string() + &format!( "/{}.bck", self.value() ) )?;

       match bincode::serialize_into( file, &self ) {
           Ok( _ ) => Ok( () ),
           Err( err ) => Err( io::Error::new( io::ErrorKind::Other, format!( "{}", err ) ) )
       }
   }
}

impl Postbox {
    pub fn respond( request: Request, server: &DynamicContentServer ) -> Response {
        let mut cookie: ValidationCookie = ValidationCookie::from_request( &request, server );

        match request.raw_headers().get( "content-type" ) {
            Some( value ) => match value.as_str() {
                "application/x-www-form-urlencoded" => match Postbox::new( server.config().postbox_directory(), &request.content() ) {
                    Ok( postbox ) => match postbox.write_file() {
                        Ok( _ ) => {
                            if cookie.validation_errors.len() > 0 {
                                cookie.validation_errors.clear();
                                cookie.contents = get_default_postbox_message();
                                cookie.save( &server.config() ).unwrap_or_else( | _ | { println!( "failed to save cookie for postbox error" ) } );
                            }
                            Response::create_and_set_redirect( 303, "/" )
                        },
                        Err( _ ) => Response::create( 500, "text/plain", "Internal server error: Could not write postbox file." ),
                    },
                    Err( postbox_error ) => match postbox_error {
                        PostboxError::MissingField( validation_errors, message ) => {
                            cookie.validation_errors = validation_errors;
                            cookie.contents = message;
                            cookie.save( &server.config() ).unwrap_or_else( | _ | { println!( "failed to save cookie for postbox error" ) } );

                            let mut response = Response::create_and_set_redirect( 303, &( String::from( server.config().reverse_proxy_prefix() ) + "/contact" ) );
                            response.set_cookie( Box::new( cookie ) );
                            response
                        },
                        PostboxError::BadPath => Response::create( 500, "text/plain", "Internal server error: bad postbox path." ),
                        PostboxError::BadParse => Response::create( 500, "text/plain", "Internal server error: Incorrect postbox format." )
                    }
                },
                _ => Response::create( 400, "text/plain", "Incorrect content-type" )
            },
            None => Response::create( 400, "text/plain", "No content-type provided" )
        }
    }

    fn new( path: &str, buffer: &str ) -> Result< Postbox, PostboxError > {
        let path = PathBuf::from( path );
        if !path.is_dir() {
            return Err( PostboxError::BadPath )
        }

        let message: PostboxMessage = match serde_urlencoded::from_str( buffer ) {
            Ok( message ) => message,
            Err( _ ) => return Err( PostboxError::BadParse )
        };

        let mut errors: Vec< String > = Vec::new();
        if message.name == "" { errors.push( "Please enter a name.".to_string() ); }
        if message.comment == "" { errors.push( "Please enter a comment.".to_string() ); }

        if errors.len() > 0 {
            Err( PostboxError::MissingField( errors, message ) )
        } else {
            Ok( Postbox { path, message } )
        }
    }

    fn write_file( &self ) -> io::Result< &str > {
        if self.message.email == "" {
            let mut file = self.path.clone();
            file.push( format!( "{}.txt", Uuid::new_v4() ) );
            fs::write( file, format!( "Name: {}\n\n{}", self.message.name, self.message.comment ) )?;
        } else {
            println!( "Postbox silently failed spam honeypot test with content {}", self.message.email );
        }

        Ok( "Success" )
    }
}
