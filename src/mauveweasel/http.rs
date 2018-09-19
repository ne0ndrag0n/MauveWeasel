use std::net::TcpStream;
use std::io::Read;
use std::collections::HashMap;
use std::io::{ BufReader, BufRead };
use mauveweasel::cookie::Cookie;
use chrono::Duration;

#[derive(Copy, Clone)]
pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE
}

pub struct Request {
    method: Method,
    url: String,
    query_string: String,
    raw_headers: HashMap< String, String >,
    content: String
}

impl Request {
    pub fn method( &self ) -> Method {
        self.method
    }

    pub fn url( &self ) -> &str {
        self.url.as_str()
    }

    pub fn query_string( &self ) -> &str {
        &self.query_string
    }

    pub fn raw_headers( &self ) -> &HashMap< String, String > {
        &self.raw_headers
    }

    pub fn content( &self ) -> String {
        self.content.clone()
    }

    pub fn from_stream( stream: &mut TcpStream, max_request_size: u64 ) -> Result< Request, &'static str > {
        let mut header_buffer: Vec< u8 > = vec![];
        let reader = BufReader::new( stream );
        let mut take = reader.take( max_request_size );

        take.read_until( b'\n', &mut header_buffer );
        let request_line = match String::from_utf8( header_buffer ) {
            Ok( result ) => result,
            Err( _ ) => return Err( "Header could not be converted to UTF-8" )
        };
        println!( "{}", request_line );
        let request_line_tokens: Vec< &str > = request_line.trim().split( ' ' ).collect();
        if request_line_tokens.len() != 3 || request_line_tokens[ 2 ] != "HTTP/1.1" {
            return Err( "Error parsing header" )
        }

        // Query string parsing
        let url_tokens: Vec< &str > = request_line_tokens[ 1 ].splitn( 2, '?' ).collect();

        let mut result = Request {
            method: match request_line_tokens[ 0 ] {
                 "GET" => Method::GET,
                 "POST" => Method::POST,
                 "PATCH" => Method::PATCH,
                 "DELETE" => Method::DELETE,
                 _ => return Err( "Invalid method" )
             },
             url: if url_tokens.len() > 0 { url_tokens[ 0 ].to_string() } else { String::new() },
             query_string: if url_tokens.len() > 1 { url_tokens[ 1 ].to_string() } else { String::new() },
             raw_headers: HashMap::new(),
             content: String::new()
        };

        // Read remaining headers
        let mut content_length = 0;
        loop {
            let mut option_buffer: Vec< u8 > = vec![];
            take.read_until( b'\n', &mut option_buffer );
            let mut option_line = match String::from_utf8( option_buffer ) {
                Ok( result ) => result,
                Err( _ ) => {
                    println!( "Could not read a header!" );
                    continue;
                }
            };

            option_line = option_line.trim().to_string();
            if option_line.len() == 0 {
                break;
            }

            let option_line_tokens: Vec< &str > = option_line.splitn( 2, ':' ).collect();
            if option_line_tokens.len() == 2 {
                match option_line_tokens[ 0 ].to_lowercase().as_str() {
                    "content-length" => {
                        content_length = match option_line_tokens[ 1 ].trim().parse::< usize >() {
                            Ok( val ) => val,
                            Err( _ ) => { println!( "Invalid value for Content-Length header!" ); 0 }
                        };
                    },
                    _ => { result.raw_headers.insert( option_line_tokens[ 0 ].to_lowercase().trim().to_string(), option_line_tokens[ 1 ].trim().to_string() ); }
                };
            }
        }

        if content_length > 0 {
            let mut buffer = vec![ 0u8; content_length ];
            match take.read_exact( &mut buffer ) {
                Ok( _ ) => {
                    match String::from_utf8( buffer ) {
                        Ok( r ) => { result.content = r; },
                        Err( _ ) => { println!( "Couldn't read content into utf8" ); }
                    }
                },
                Err( _ ) => { println!( "Couldn't read content" ); }
            };
        }

        Ok( result )
    }

}

pub struct Response {
    code: u16,
    content_type: String,
    redirect: String,
    body: String,
    cookies: Vec< Box< Cookie > >
}

impl Response {
    pub fn redirect( &self ) -> &str {
        &self.redirect
    }

    pub fn set_redirect( &mut self, path: &str ) {
        self.redirect = path.to_string();
    }

    pub fn set_cookie( &mut self, cookie: Box< Cookie > ) {
        self.cookies.push( cookie );
    }

    pub fn create( code: u16, content_type: &str, body: &str ) -> Response {
        Response { code, content_type: content_type.to_string(), redirect: String::new(), body: body.to_string(), cookies: Vec::new() }
    }

    pub fn create_and_set_redirect( code: u16, redirect: &str ) -> Response {
        Response { code, content_type: "text/plain".to_string(), redirect: redirect.to_string(), body: String::new(), cookies: Vec::new() }
    }

    fn generate_headers( &self ) -> String {
        let mut more_headers = String::new();
        if self.redirect != "" {
            more_headers += &format!( "Location: {}\r\n", self.redirect );
        }

        for boxed_cookie in &self.cookies {
            more_headers += &format!(
                "Set-Cookie: {}={};{}\r\n",
                boxed_cookie.name(),
                boxed_cookie.value(),
                match boxed_cookie.get_expiry() {
                    Some( expiry ) => format!( " Max-Age:{}", expiry ),
                    None => "".to_string()
                }
            );
        }

        more_headers
    }

    pub fn generate( &self ) -> Vec<u8> {
        Vec::from( format!(
            "HTTP/1.1 {} \r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\r\n{}",
            self.code, self.content_type, self.body.len(),
            self.generate_headers(),
            self.body
        ).as_bytes() )
    }
}
