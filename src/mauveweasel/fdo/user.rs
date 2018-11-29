use uuid::Uuid;
use serde::ser::{ Serialize, Serializer };

#[derive(Serialize,Deserialize)]
pub struct User {
    uuid: String,
    username: String
}

impl User {

    pub fn new( username: &str ) -> User {
        User {
            uuid: format!( "{}", Uuid::new_v4() ),
            username: username.to_owned()
        }
    }

}
