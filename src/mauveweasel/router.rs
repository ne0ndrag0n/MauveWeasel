use mauveweasel::options::Config;
use mauveweasel::components::postbox::Postbox;
use mauveweasel::http::{Method, Request,Response};

pub fn route( request: Request, config: &Config ) -> Response {
    match ( request.method(), request.url() ) {
        ( Method::GET, "/status" ) => Response::create( 200, "text/plain", "up" ),
        ( Method::POST, "/postbox" ) => match request.raw_headers().get( "content-type" ) {
            Some( value ) => match value.as_str() {
                "application/x-www-form-urlencoded" => match Postbox::new( config.postbox_directory() ) {
                    Ok( postbox ) => match postbox.write_file( request.content() ) {
                        Ok( message ) => {
                            let mut response = Response::create( 303, "text/plain", "" );
                            response.set_redirect( "https://www.ne0ndrag0n.com/" );
                            response
                        },
                        Err( error ) => Response::create( 500, "text/plain", &format!( "Error: {}", error ) )
                    },
                    Err( message ) => Response::create( 500, "text/plain", message )
                },
                _ => Response::create( 400, "text/plain", "Incorrect content-type" )
            },
            None => Response::create( 400, "text/plain", "No content-type provided" )
        },
        ( _, _ ) => Response::create( 404, "text/plain", "Not found" )
    }
}
