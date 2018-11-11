use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn get_file_string( path: &str ) -> Result< String > {
    let mut file = File::open( path )?;
    let mut result = String::new();

    file.read_to_string( &mut result )?;

    Ok( result )
}
