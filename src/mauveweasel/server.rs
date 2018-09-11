use mauveweasel::options::Config;
use mauveweasel::utility;
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

    }
}
