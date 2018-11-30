use uuid::Uuid;
use std::io;
use mauveweasel::fdo::datalayer::{ FileDataLayer, FdoStoredObject, FdoObject };

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

impl<'a> FdoObject<'a> for User {

    fn type_key() -> &'static str {
        "user"
    }

    fn create( self, fdo: &'a mut FileDataLayer ) -> io::Result< &'a mut User > {
        Ok(
            fdo.create(
                self.uuid().to_owned(),
                Self::type_key(),
                FdoStoredObject::User( self )
            )?.as_user_mut()
        )
    }

    fn retrieve( uuid: String, fdo: &'a mut FileDataLayer ) -> io::Result< &'a mut User > {
        Ok(
            fdo.retrieve( uuid, Self::type_key() )?.as_user_mut()
        )
    }

    fn update( &self, fdo: &mut FileDataLayer ) -> io::Result< () > {
        Err( std::io::Error::new(std::io::ErrorKind::Other, "oh no!" ) )
    }

    fn delete( self, fdo: &mut FileDataLayer ) -> io::Result< () > {
        Err( std::io::Error::new(std::io::ErrorKind::Other, "oh no!" ) )
    }

}
