use mauveweasel::options::Config;
use mauveweasel::utility;
use toml;
use std::net::{TcpStream,TcpListener};
use std::io::Read;

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

impl DynamicContentServer {

    pub fn new() -> DynamicContentServer {
        let result = DynamicContentServer {
            config: toml::from_str( utility::get_file_string( "options.toml" ).as_str() )
                          .expect( "Could not parse TOML" )
        };

        result
    }

    fn build_request( &self, stream: &mut TcpStream ) {
        match stream_to_string( stream ) {
            Ok( result ) => {

            },
            Err( message ) => eprintln!( "build_request failed: {}", message )
        }
    }

    pub fn run( &self ) {
        let listener = TcpListener::bind( self.config.get_host() ).expect( "Failed to set up a TcpListener!" );

        for stream in listener.incoming() {
            match stream {
                Ok( mut stream ) => {
                    self.build_request( &mut stream );
                },
                Err( e ) => println!( "Unable to connect: {}", e )
            }
        }
    }
}
