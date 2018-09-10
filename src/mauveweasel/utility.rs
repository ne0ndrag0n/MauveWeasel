use std::fs::File;
use std::io::Read;

pub fn get_file_string( path: &str ) -> String {
    let mut file = File::open( path ).expect( &format!( "Unable to open file: {}", path ) );
    let mut result = String::new();

    file.read_to_string( &mut result ).expect( &format!( "Unable to read file: {}", path ) );

    result
}
