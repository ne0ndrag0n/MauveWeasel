use mauveweasel::handler::Handler;
use tiny_http::{Request, Response};
use std::vec::Vec;
use std::io::Cursor;
use std::path::PathBuf;
use std::result::Result;
use std::io;
use std::fs;
use uuid::Uuid;

pub struct Postbox {
    path: PathBuf
}

impl Postbox {
    pub fn new( path: &str ) -> Result< Postbox, &'static str > {
        let path = PathBuf::from( path );
        match path.is_dir() {
            true => Ok( Postbox { path } ),
            false => Err( "Invalid path for postbox" )
        }
    }

    pub fn write_file( &self, buffer: String ) -> io::Result< &str > {
        let mut result = String::new();

        let pairs = buffer.split( '&' );
        for pair in pairs {
            let pair: Vec< &str > = pair.split( '=' ).collect();
            if pair.len() == 2 {
                result += &pair.join( ": " );
                result += "\n";
            }
        }

        if result.len() > 0 {
            let mut file = self.path.clone();
            file.push( format!( "{}.txt", Uuid::new_v4() ) );
            fs::write( file, result )?;
        }

        Ok( "Success" )
    }
}

impl Handler for Postbox {
    fn handle( self, request: &mut Request ) -> Response< Cursor < Vec < u8 > > > {
        let headers = Vec::from( request.headers() );
        for header in headers {
            if header.field.equiv( "Content-Type" ) && header.value == "application/x-www-form-urlencoded" {
                let mut content = String::new();
                if request.as_reader().read_to_string( &mut content ).is_ok() {
                    return match self.write_file( content ) {
                        Ok( message ) => Response::from_string( message ),
                        Err( message ) => Response::from_string( format!( "{}", message ) ).with_status_code( 500 )
                    }
                }
            }
        }

        Response::from_string( "No Content-Type or unexpected MIME type!" ).with_status_code( 400 )
    }
}
