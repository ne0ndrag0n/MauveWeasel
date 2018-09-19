use mauveweasel::server::DynamicContentServer;
use mauveweasel::components::postbox::ValidationCookie;
use mauveweasel::cookie::Cookie;
use mauveweasel::http::{Request, Response};
use serde_urlencoded;

#[derive(Deserialize)]
pub struct CommentValidation {
    pub name_valid: Option< bool >,
    pub comment_valid: Option< bool >
}

fn get_validation( query_string: &str ) -> Vec< &str > {
    let validation = match serde_urlencoded::from_str( query_string ) {
        Ok( good ) => good,
        Err( _ ) => CommentValidation{ name_valid: Some( true ), comment_valid: Some( true ) }
    };

    let mut result = Vec::new();
    if validation.name_valid.is_some() && validation.name_valid.unwrap() == false {
        result.push( "Please provide a name." );
    }

    if validation.comment_valid.is_some() && validation.comment_valid.unwrap() == false {
        result.push( "Please provide a comment." );
    }

    result
}

pub fn respond( request: Request, server: &DynamicContentServer ) -> Response {
    // TODO: Retrieve postbox message from cookie and repopulate form
    let cookie: Box< Cookie > = Box::new( ValidationCookie::from_request( &request, server ) );

    match server.templates().render( "contact", &json!( { "validation_errors": get_validation( request.query_string() ) } ) ) {
        Ok( string ) => Response::create( 200, "text/html", &string ),
        Err( string ) => Response::create( 500, "text/plain", &format!( "error: {}", string ) )
    }
}
