use mauveweasel::options::Config;
use mauveweasel::utility;
use mauveweasel::http::Request;
use mauveweasel::router;
use toml;
use std::net::{TcpListener};
use std::io::Write;
use handlebars::Handlebars;
use std::path::Path;

pub struct DynamicContentServer {
    config: Config,
    templates: Handlebars
}

impl DynamicContentServer {

    pub fn config( &self ) -> &Config {
        &self.config
    }

    pub fn templates( &self ) -> &Handlebars {
        &self.templates
    }

    pub fn new() -> DynamicContentServer {
        let mut result = DynamicContentServer {
            config: toml::from_str( utility::get_file_string( "/etc/mauveweasel/options.toml" ).as_str() )
                          .expect( "Could not parse TOML" ),
            templates: Handlebars::new()
        };

        // Register templates in templates directory
        match result.templates.register_templates_directory(
            ".hbs",
            Path::new( result.config.templates_directory() )
        ) {
            Ok( _ ) => println!( "Successfully loaded templates" ),
            Err( message ) => println!( "Error loading templates: {}", message )
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
                            stream.write(
                                &router::route( request, &self ).generate()
                            ).expect( "Response failed" );
                        },
                        Err( message ) => println!( "Unable to connect: {}", message )
                    }
                },
                Err( e ) => println!( "Unable to connect: {}", e )
            }
        }
    }
}
