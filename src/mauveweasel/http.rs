use std::vec::Vec;

pub enum Method {
    Unknown,
    GET,
    POST,
    PATCH,
    DELETE
}

pub struct Request {
    method: Method,
    url: String
}

impl Request {
    pub fn from_string( input: String ) -> Result< Request, &'static str > {
        let mut result = Request {
            method: Method::Unknown,
            url: String::from( "" )
        };
        let mut lines = input.lines();
        let http_request_header: Vec< &str > = match lines.next() {
            Some( result ) => result.split( ' ' ).collect(),
            None => return Err( "Malformed http header" )
        };

        if http_request_header.len() != 3 || http_request_header[ 2 ] != "HTTP/1.1" {
            return Err( "Malformed http header" )
        }

        result.method = match http_request_header[ 0 ] {
             "GET" => Method::GET,
             "POST" => Method::POST,
             "PATCH" => Method::PATCH,
             "DELETE" => Method::DELETE,
             _ => return Err( "Invalid method" )
         };

        result.url += http_request_header[ 1 ];

        Ok( result )
    }
}
