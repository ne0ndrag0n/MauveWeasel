use std::net::TcpStream;
use std::io::Read;
use std::collections::HashMap;
use std::io::{ BufReader, BufRead };

pub enum Method {
    Unknown,
    GET,
    POST,
    PATCH,
    DELETE
}

pub struct Request {
    method: Method,
    url: String,
    raw_headers: HashMap< String, String >,
    content: String
}

impl Request {
    pub fn from_stream( stream: &mut TcpStream, max_request_size: u64 ) -> Result< Request, &'static str > {
        let mut header_buffer: Vec< u8 > = vec![];
        let reader = BufReader::new( stream );
        let mut take = reader.take( max_request_size );

        take.read_until( b'\n', &mut header_buffer );
        let request_line = match String::from_utf8( header_buffer ) {
            Ok( result ) => result,
            Err( _ ) => return Err( "Header could not be converted to UTF-8" )
        };
        let request_line_tokens: Vec< &str > = request_line.trim().split( ' ' ).collect();
        if request_line_tokens.len() != 3 || request_line_tokens[ 2 ] != "HTTP/1.1" {
            return Err( "Error parsing header" )
        }

        let mut result = Request {
            method: match request_line_tokens[ 0 ] {
                 "GET" => Method::GET,
                 "POST" => Method::POST,
                 "PATCH" => Method::PATCH,
                 "DELETE" => Method::DELETE,
                 _ => return Err( "Invalid method" )
             },
             url: request_line_tokens[ 1 ].to_string(),
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
                    _ => { result.raw_headers.insert( option_line_tokens[ 0 ].trim().to_string(), option_line_tokens[ 1 ].trim().to_string() ); }
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
