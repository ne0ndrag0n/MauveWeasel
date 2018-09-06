use mauveweasel::settings::Settings;
use hyper;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use std::fs::File;
use std::io::Read;
use toml;

pub struct MauveWeaselServer {
    settings: Settings
}

impl MauveWeaselServer {
    pub fn new() -> MauveWeaselServer {
        MauveWeaselServer {
            settings: {
                let mut settings = File::open( "./settings.toml" ).expect( "settings.toml not found" );
                let mut content = String::new();
                settings.read_to_string( &mut content ).expect( "failed to read settings.toml" );
                toml::from_str( &content ).expect( "failed to parse settings.toml" )
            }
        }
    }

    pub fn run( &self ) {
        let addr = ( [ 127, 0, 0, 1 ], self.settings.port() ).into();
        let server = Server::bind( &addr )
            .serve( || {
                service_fn_ok( |_| {
                    Response::new( Body::from( "request me uwu" ) )
                } )
            } )
            .map_err( | e | {
                eprintln!( "server error: {}", e );
            } );

        hyper::rt::run( server );
    }
}
