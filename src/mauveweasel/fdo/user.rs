use mauveweasel::options::Config;
use mauveweasel::fdo::datalayer;
use mauveweasel::fdo::datalayer::FdoObject;
use uuid::Uuid;
use std::io;

#[derive(Serialize,Deserialize)]
pub struct User {
    uuid: String,
    username: String
}

impl User {

    fn new( username: &str ) -> User {
        User {
            uuid: format!( "{}", Uuid::new_v4() ),
            username: username.to_owned()
        }
    }

    fn uuid( &self ) -> &str {
        &self.uuid
    }

}

impl FdoObject for User {
    fn key() -> &'static str where User: Sized {
        "user"
    }

    fn uuid( &self ) -> &str {
        &self.uuid
    }

    fn retrieve( uuid: &str, config: &Config ) -> io::Result< User > where User: Sized {
        datalayer::load( uuid, config )
    }

    fn save( &self, config: &Config ) -> io::Result< () > {
        datalayer::save( self, config )
    }

    fn delete( &self, config: &Config ) -> io::Result< () > {
        datalayer::remove( self, config )
    }
}
