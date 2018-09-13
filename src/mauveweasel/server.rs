use mauveweasel::options::Config;
use mauveweasel::utility;
use mauveweasel::http::Request;
use toml;
use std::net::{TcpStream,TcpListener};
use std::io::Read;
use std::io::Write;

pub struct DynamicContentServer {
    config: Config
}

fn stream_to_string( stream: &mut TcpStream ) -> Result< String, &'static str > {
    let mut buf = vec![];
    match stream.read_to_end( &mut buf ) {
        Ok( bytes ) => match String::from_utf8( buf ) {
                Ok( string ) => Ok( string ),
                Err( e ) => Err( "Failed to convert to utf8" )
        },
        Err( e ) => Err( "Failed to convert tcp stream to string" )
    }
}

fn build_request( stream: &mut TcpStream ) -> Result< Request, &'static str > {
    match stream_to_string( stream ) {
        Ok( result ) => Request::from_string( result ),
        Err( message ) => Err( message )
    }
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
                    match build_request( &mut stream ) {
                        Ok( request ) => {
                            request.__debug_printstring();
                            stream.write( b"response" ).expect( "Response failed" );
                        },
                        Err( message ) => eprintln!( "Unable to connect: {}", message )
                    }
                },
                Err( e ) => eprintln!( "Unable to connect: {}", e )
            }
        }
    }
}
