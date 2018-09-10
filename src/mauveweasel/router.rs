use tiny_http::{Request, Response,Method};
use mauveweasel::handler::Handler;
use mauveweasel::options::Config;
use mauveweasel::components::postbox::Postbox;
use std::io::Cursor;

fn route_post( request: &mut Request, config: &Config ) -> Response< Cursor < Vec < u8 > > >  {
    match request.url() {
        "/postbox" => Postbox::new( &config.postbox_directory() )
                               .expect( "Could not create a Postbox!" )
                               .handle( request ),
        _ => route_error( 404 )
    }
}

fn route_error( code: u16 ) -> Response< Cursor < Vec < u8 > > >  {
    Response::from_string( "Error!" ).with_status_code( code )
}

pub fn route( mut request: Request, config: &Config ) {
    let response = match request.method() {
        Method::Post => route_post( &mut request, config ),
        _ => route_error( 405 )
    };

    request.respond( response ).expect( "Failed to respond to request" );
}
