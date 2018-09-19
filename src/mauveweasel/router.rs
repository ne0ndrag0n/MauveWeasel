use mauveweasel::server::DynamicContentServer;
use mauveweasel::components::postbox::Postbox;
use mauveweasel::components::contactform as ContactForm;
use mauveweasel::http::{Method, Request,Response};

pub fn route( request: Request, server: &DynamicContentServer ) -> Response {
    match ( request.method(), request.url() ) {
        ( Method::GET, "/status" ) => Response::create( 200, "text/plain", "up" ),
        ( Method::GET, "/contact" ) => ContactForm::respond( request, server ),
        ( Method::POST, "/postbox" ) => Postbox::respond( request, server ),
        ( _, _ ) => Response::create( 404, "text/plain", "Not found" )
    }
}
