use bincode;
use uuid::Uuid;
use std::io;
use std::fs::OpenOptions;
use std::collections::HashMap;
use mauveweasel::options::Config;

type Keystore = HashMap< String, String >;

fn invert_map( orig: &Keystore ) -> HashMap< &str, &str > {
    let mut result: HashMap< &str, &str > = HashMap::new();

    for ( key, value ) in orig {
        result.insert( &value, &key );
    }

    result
}

fn load_store( config: &Config ) -> io::Result< Keystore > {
    let file = OpenOptions::new().read( true ).write( true ).create( true ).open( format!( "{}/keystore.bin", config.data_directory() ) )?;

    match bincode::deserialize_from( file ) {
        Ok( keystore ) => Ok( keystore ),
        Err( er ) => { println!( "{:?}", er ); Ok( Keystore::new() ) }
    }
}

fn save_store( keystore: &Keystore, config: &Config ) -> io::Result< () > {
    let file = OpenOptions::new().read( true ).write( true ).create( true ).open( format!( "{}/keystore.bin", config.data_directory() ) )?;

    match bincode::serialize_into( &file, keystore ) {
        Ok( _ ) => Ok( () ),
        Err( _ ) => Err( io::Error::new( io::ErrorKind::Other, "" ) )
    }
}

pub fn get_or_associate_uuid( value: &str, config: &Config ) -> Result< String, &'static str > {
    // Load keystore file
    let mut keystore = match load_store( config ) {
        Ok( keystore ) => keystore,
        Err( _ ) => return Err( "Could not load keystore" )
    };

    // See if it's already in the map
    match invert_map( &keystore ).get( value ) {
        Some( uuid ) => return Ok( uuid.to_string() ),
        None => {}
    };

    // Key needs to be created
    let uuid = format!( "{}", Uuid::new_v4() ).to_string();
    keystore.insert( uuid.clone(), value.to_string() );

    match save_store( &keystore, config ) {
        Ok( _ ) => Ok( uuid ),
        Err( _ ) => Err( "Could not save keystore" )
    }
}

pub fn get_value( uuid: &str, config: &Config ) -> Result< Option< String >, &'static str > {
    // Load keystore file
    let keystore = match load_store( config ) {
        Ok( keystore ) => keystore,
        Err( _ ) => return Err( "Could not load keystore" )
    };

    match keystore.get( uuid ) {
        Some( value ) => Ok( Some( value.clone() ) ),
        None => Ok( None )
    }
}
