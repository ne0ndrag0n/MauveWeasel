use mauveweasel::server::DynamicContentServer;
use mauveweasel::components::postbox::ValidationCookie;
use mauveweasel::http::{Request, Response};

#[derive(Deserialize)]
pub struct CommentValidation {
    pub name_valid: Option< bool >,
    pub comment_valid: Option< bool >
}

pub fn respond( request: Request, server: &DynamicContentServer ) -> Response {
    let cookie: ValidationCookie = ValidationCookie::from_request( &request, server );

    match server.templates().render(
        "contact",
        &json!( {
            "validation_errors": cookie.validation_errors,
            "name": cookie.contents.name,
            "comment": cookie.contents.comment
        } )
    ) {
        Ok( string ) => Response::create( 200, "text/html", &string ),
        Err( string ) => Response::create( 500, "text/plain", &format!( "error: {}", string ) )
    }
}
