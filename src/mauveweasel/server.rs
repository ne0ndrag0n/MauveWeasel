use mauveweasel::comment::Comment;
use mauveweasel::options::Config;
use mauveweasel::utility;
use toml;

pub fn run() {
    let config: Config =
        toml::from_str(
            utility::get_file_string( "options.toml" ).as_str()
        ).expect( "Could not parse TOML" );

    let addr = config.get_host();
    println!( "{}", addr );
}
