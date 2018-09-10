use mauveweasel::comment::Comment;
use mauveweasel::options::Config;
use mauveweasel::utility;
use tiny_http::{Server, Request, Response};
use mauveweasel::router;
use toml;

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
        let server = Server::http( self.config.get_host() ).expect( "Could not create server" );
        for request in server.incoming_requests() {
            router::route( request, &self.config );
        }
    }
}
