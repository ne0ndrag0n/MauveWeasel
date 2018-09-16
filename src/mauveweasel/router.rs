use mauveweasel::server::DynamicContentServer;
use mauveweasel::components::postbox::PostboxError;
use mauveweasel::components::postbox::Postbox;
use mauveweasel::components::contactform;
use mauveweasel::http::{Method, Request,Response};

pub fn route( request: Request, server: &DynamicContentServer ) -> Response {
    match ( request.method(), request.url() ) {
        ( Method::GET, "/status" ) => Response::create( 200, "text/plain", "up" ),
        ( Method::GET, "/contact" ) =>
        match server.templates()
            .render(
                "contact",
                &json!( {
                    "validation_errors": contactform::get_validation( request.query_string() )
                } )
            ) {
            Ok( string ) => Response::create( 200, "text/html", &string ),
            Err( string ) => Response::create( 500, "text/plain", &format!( "error: {}", string ) )
        },
        ( Method::POST, "/postbox" ) => match request.raw_headers().get( "content-type" ) {
            Some( value ) => match value.as_str() {
                "application/x-www-form-urlencoded" => match Postbox::new( server.config().postbox_directory(), &request.content() ) {
                    Ok( postbox ) => match postbox.write_file() {
                        Ok( _ ) => Response::create_and_set_redirect( 303, "/" ),
                        Err( _ ) => Response::create( 500, "text/plain", "Internal server error: Could not write postbox file." ),
                    },
                    Err( postbox_error ) => match postbox_error {
                        PostboxError::MissingName => Response::create_and_set_redirect( 303, &( String::from( server.config().reverse_proxy_prefix() ) + "/contact?name_valid=false" ) ),
                        PostboxError::MissingComment => Response::create_and_set_redirect( 303, &( String::from( server.config().reverse_proxy_prefix() ) + "/contact?comment_valid=false" ) ),
                        PostboxError::BadPath => Response::create( 500, "text/plain", "Internal server error: bad postbox path." ),
                        PostboxError::BadParse => Response::create( 500, "text/plain", "Internal server error: Incorrect postbox format." )
                    }
                },
                _ => Response::create( 400, "text/plain", "Incorrect content-type" )
            },
            None => Response::create( 400, "text/plain", "No content-type provided" )
        },
        ( _, _ ) => Response::create( 404, "text/plain", "Not found" )
    }
}
