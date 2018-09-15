use mauveweasel::options::Config;
use mauveweasel::http::{Method, Request,Response};

pub fn route( request: Request, config: &Config ) -> Response {
    match ( request.method(), request.url() ) {
        ( Method::GET, "/status" ) => Response::create( 200, "text/plain", "up" ),
        ( _, _ ) => Response::create( 404, "text/plain", "Not found" )
    }
}
