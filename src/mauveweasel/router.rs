use mauveweasel::server::DynamicContentServer;
use mauveweasel::components::postbox::Postbox;
use mauveweasel::components::contactform as ContactForm;
use mauveweasel::http::{Method, Request,Response};

enum UrlTokenType {
    MatchingConstant,
    MatchingVariable( String )
}

fn get_token( format_segment: &str, url_segment: &str ) -> Option< UrlTokenType > {
    match format_segment.chars().nth( 0 ) {
        Some( segment ) => match segment {
            ':' => return Some( UrlTokenType::MatchingVariable( url_segment.to_string() ) ),
            _ => {
                if format_segment == url_segment {
                    return Some( UrlTokenType::MatchingConstant )
                } else {
                    return None
                }
            }
        },
        None => match url_segment.chars().nth( 0 ) {
            None => return Some( UrlTokenType::MatchingConstant ),
            Some( _ ) => /* WTF */ return None
        }
    }
}

fn match_dynamic_url( format: &str, url: String ) -> Option< Vec< String > > {
    let mut result = vec![];
    let format: Vec< &str > = format.split( "/" ).collect();
    let url: Vec< &str > = url.split( "/" ).collect();

    if format.len() != url.len() {
        return None
    }

    for i in 0..format.len() {
        match get_token( format[ i ], url[ i ] ) {
            Some( token_type ) => match token_type {
                UrlTokenType::MatchingConstant => { /* pass */ },
                UrlTokenType::MatchingVariable( variable ) => {
                    result.push( variable );
                }
            },
            None => return None
        }
    }

    Some( result )
}

pub fn route( request: Request, server: &DynamicContentServer ) -> Response {
    let method = request.method();
    let url = request.url().to_string();

    match ( method, url.as_str() ) {
        ( Method::GET, "/status" ) => Response::create( 200, "text/plain", "up" ),
        ( Method::GET, "/contact" ) => ContactForm::respond( request, server ),
        ( Method::POST, "/postbox" ) => Postbox::respond( request, server ),
        ( Method::GET, dynamic_url ) => {
            Response::create( 501, "text/plain", "Not implemented" )
        }
        ( _, _ ) => Response::create( 404, "text/plain", "Not found" )
    }
}
