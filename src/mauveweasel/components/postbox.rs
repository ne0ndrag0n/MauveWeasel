use std::vec::Vec;
use std::path::PathBuf;
use std::result::Result;
use std::io;
use std::fs;
use mauveweasel::cookie;
use mauveweasel::cookie::Cookie;
use uuid::Uuid;
use serde_urlencoded;
use bincode::{serialize,deserialize};

pub enum PostboxError {
    MissingName,
    MissingComment,
    BadPath,
    BadParse
}

pub struct Postbox {
    path: PathBuf,
    message: PostboxMessage
}

#[derive(Clone,Serialize,Deserialize)]
struct PostboxMessage {
    pub name: String,
    pub comment: String,
    pub email: String
}

#[derive(Serialize,Deserialize)]
struct ValidationCookie {
   id: String,
   contents: PostboxMessage
}

impl Cookie for ValidationCookie {
   fn name( &self ) -> &str { "postbox-validation-cookie" }
   fn value( &self ) -> &str { &self.id }
   fn save( &self ) {

   }
}

impl Postbox {
    pub fn new( path: &str, buffer: &str ) -> Result< Postbox, PostboxError > {
        let path = PathBuf::from( path );
        if !path.is_dir() {
            return Err( PostboxError::BadPath )
        }

        let message: PostboxMessage = match serde_urlencoded::from_str( buffer ) {
            Ok( message ) => message,
            Err( _ ) => return Err( PostboxError::BadParse )
        };

        if message.name == "" { return Err( PostboxError::MissingName ) }
        if message.comment == "" { return Err( PostboxError::MissingComment ) }

        Ok( Postbox { path, message } )
    }

    pub fn write_file( &self ) -> io::Result< &str > {
        if self.message.email == "" {
            let mut file = self.path.clone();
            file.push( format!( "{}.txt", Uuid::new_v4() ) );
            fs::write( file, format!( "Name: {}\n\n{}", self.message.name, self.message.comment ) )?;
        } else {
            println!( "Postbox silently failed spam honeypot test with content {}", self.message.email );
        }

        Ok( "Success" )
    }
}
