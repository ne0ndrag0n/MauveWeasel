use mauveweasel::fdo::user::User;
use mauveweasel::options::Config;
use std::io;
use std::fs::File;
use bincode;
use lru_cache::LruCache;

pub trait FdoObject<'a> {
    fn type_key() -> &'static str;

    fn create( self, fdo: &'a mut FileDataLayer ) -> io::Result< &'a mut Self >;

    fn retrieve( uuid: String, fdo: &'a mut FileDataLayer ) -> io::Result< &'a mut Self >;

    fn update( &self, fdo: &mut FileDataLayer ) -> io::Result< () >;

    fn delete( self, fdo: &mut FileDataLayer ) -> io::Result< () >;
}

#[derive(Serialize,Deserialize)]
pub enum FdoStoredObject {
    User( User ),
    Other
}

impl FdoStoredObject {

    pub fn as_user( &self ) -> &User {
        match self {
            FdoStoredObject::User( user ) => &user,
            _ => panic!( "Type mismatch" )
        }
    }

    pub fn as_user_mut( &mut self ) -> &mut User {
        match self {
            FdoStoredObject::User( user ) => user,
            _ => panic!( "Type mismatch" )
        }
    }

}

pub struct FileDataLayer<'a> {
    cache: LruCache< String, FdoStoredObject >,
    data_directory: &'a str
}

impl<'a> FileDataLayer<'a> {

    pub fn new( config: &Config ) -> FileDataLayer {
        FileDataLayer {
            cache: LruCache::new( config.newsgen_lru_cache_size() ),
            data_directory: config.data_directory()
        }
    }

    pub fn create( &mut self, uuid: String, key: &'static str, object: FdoStoredObject ) -> io::Result< &mut FdoStoredObject > {
        // Save file
        match bincode::serialize_into( File::open( format!( "{}/{}/{}.bin", self.data_directory, key, uuid ) )?, &object ) {
            Ok( _ ) => {},
            Err( _ ) => return Err( io::Error::new( io::ErrorKind::Other, "" ) )
        };

        // Insert into cache
        let uuid_copy = uuid.to_owned();
        self.cache.insert( uuid, object );
        Ok( self.cache.get_mut( &uuid_copy ).unwrap() )
    }

    pub fn retrieve( &mut self, uuid: String, key: &'static str ) -> io::Result< &mut FdoStoredObject > {
        if self.cache.contains_key( &uuid ) {
            return Ok( self.cache.get_mut( &uuid ).unwrap() )
        }

        // Gotta load the file off disk
        let product = match bincode::deserialize_from( File::open( format!( "{}/{}/{}.bin", self.data_directory, key, uuid ) )? ) {
            Ok( product ) => product,
            Err( _ ) => return Err( io::Error::new( io::ErrorKind::Other, "" ) )
        };

        let uuid_copy = uuid.to_owned();
        self.cache.insert( uuid, product );
        Ok( self.cache.get_mut( &uuid_copy ).unwrap() )
    }

}
