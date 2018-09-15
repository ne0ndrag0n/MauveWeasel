use std::vec::Vec;
use std::path::PathBuf;
use std::result::Result;
use std::io;
use std::fs;
use uuid::Uuid;
use serde_urlencoded;

pub struct Postbox {
    path: PathBuf
}

#[derive(Deserialize)]
struct PostboxMessage {
    pub name: String,
    pub comment: String,
    pub honeypot: String
}

impl Postbox {
    pub fn new( path: &str ) -> Result< Postbox, &'static str > {
        let path = PathBuf::from( path );
        match path.is_dir() {
            true => Ok( Postbox { path } ),
            false => Err( "Invalid path for postbox" )
        }
    }

    pub fn write_file( &self, buffer: String ) -> io::Result< &str > {
        let message: PostboxMessage = match serde_urlencoded::from_str( &buffer ) {
            Ok( message ) => message,
            Err( _ ) => return Err( io::Error::new( io::ErrorKind::Other, "Can't parse x-www-url-formencoded" ) )
        };

        if message.name == "" { return Err( io::Error::new( io::ErrorKind::Other, "Missing field: Name" ) ); }
        if message.comment == "" { return Err( io::Error::new( io::ErrorKind::Other, "Missing field: Comment" ) ); }

        if message.honeypot == "" {
            let mut file = self.path.clone();
            file.push( format!( "{}.txt", Uuid::new_v4() ) );
            fs::write( file, format!( "Name: {}\nComment: {}\n", message.name, message.comment ) )?;
        } else {
            println!( "Postbox silently failed spam honeypot test with content {}", message.honeypot );
        }

        Ok( "Success" )
    }
}
