use mauveweasel::options::Config;
use mauveweasel::utility;
use toml;
use hyper::{ Request,Response,Body,Server,Error };
use hyper::service::service_fn_ok;
use hyper::rt::{ self, Future };
use hyper::service::Service;
use futures::future::{ self, FutureResult };

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

    fn s( &self ) {

    }

    pub fn run( &self ) {
        /*
        let addr = self.config.get_host().parse().expect( "Incorrect format for IP" );

        let server = Server::bind( &addr )
        .serve( | | {
            service_fn_ok(| request | {
                Response::new( Body::from( "oof" ) )
            })
        } )
        .map_err( | e | eprintln!( "server error: {}", e ) );

        rt::run( server );
        */
    }
}


impl Service for DynamicContentServer {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Future = FutureResult< Response< Body >, Self::Error >;

    fn call( &mut self, request: Request< Body > ) -> Self::Future {
        let response = Response::new( "oof".into() );
        future::ok( response )
    }
}
