use mauveweasel::options::Config;
use std::io;
use std::fs::{ File, OpenOptions, remove_file };
use serde::ser::Serialize;
use serde::de::Deserialize;
use bincode;

pub trait FdoObject {
    fn key() -> &'static str where Self: Sized;

    fn uuid( &self ) -> &str;

    fn retrieve( uuid: &str, config: &Config ) -> io::Result< Self > where Self: Sized;

    fn save( &self, config: &Config ) -> io::Result< () >;

    fn delete( &self, config: &Config ) -> io::Result< () >;
}

fn delete_file( key: &'static str, uuid: &str, config: &Config ) -> io::Result< () > {
    remove_file( format!( "{}/{}/{}.bin", config.data_directory(), key, uuid ) )
}

fn open_file( key: &'static str, uuid: &str, config: &Config ) -> io::Result< File > {
    OpenOptions::new().read( true ).write( true ).create( true ).open( format!( "{}/{}/{}.bin", config.data_directory(), key, uuid ) )
}

pub fn save< 'de, FdoDerivative >( obj: &FdoDerivative, config: &Config ) -> io::Result< () > where FdoDerivative: FdoObject + Serialize + Deserialize<'de> {
    let file = open_file( FdoDerivative::key(), obj.uuid(), config )?;

    match bincode::serialize_into( file, obj ) {
        Ok( _ ) => Ok( () ),
        Err( _ ) => Err( io::Error::new( io::ErrorKind::Other, "" ) )
    }
}

pub fn load< FdoDerivative >( uuid: &str, config: &Config ) -> io::Result< FdoDerivative > where for<'de> FdoDerivative: FdoObject + Serialize + Deserialize<'de> {
    let file = open_file( FdoDerivative::key(), uuid, config )?;

    match bincode::deserialize_from( file ) {
        Ok( result ) => Ok( result ),
        Err( _ ) => Err( io::Error::new( io::ErrorKind::Other, "" ) )
    }
}

pub fn remove< FdoDerivative >( obj: &FdoDerivative, config: &Config ) -> io::Result< () > where FdoDerivative: FdoObject {
    delete_file( FdoDerivative::key(), obj.uuid(), config )
}
