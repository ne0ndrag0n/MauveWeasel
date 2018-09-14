use mauveweasel::options::Config;
use mauveweasel::utility;
use mauveweasel::http::Request;
use toml;
use std::net::{TcpListener};
use std::io::Write;

pub struct DynamicContentServer {
    config: Config
}

impl DynamicContentServer {

    pub fn new() -> DynamicContentServer {
        let result = DynamicContentServer {
            config: toml::from_str( utility::get_file_string( "options.toml" ).as_str() )
                          .expect( "Could not parse TOML" )
        };

        result
    }

    pub fn run( &self ) {
        let listener = TcpListener::bind( self.config.get_host() ).expect( "Failed to set up a TcpListener!" );

        for stream in listener.incoming() {
            match stream {
                Ok( mut stream ) => {
                    match Request::from_stream( &mut stream, self.config.max_request_size() ) {
                        Ok( request ) => {
                            stream.write( b"response\n" ).expect( "Response failed" );
                        },
                        Err( message ) => println!( "Unable to connect: {}", message )
                    }
                },
                Err( e ) => println!( "Unable to connect: {}", e )
            }
        }
    }
}
