use mauveweasel::fdo::user::User;
use std::thread_local;
use std::cell::RefCell;
use std::io;

trait FdoObject<'a> {
    fn create( fdo: &FileDataLayer ) -> Self;

    fn retrieve( uuid: &str, fdo: &'a FileDataLayer ) -> &'a Self;

    fn update( &self, fdo: &FileDataLayer ) -> io::Result< () >;

    fn delete( &self ) -> io::Result< () >;
}

enum FdoStoredObject {
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

}

pub struct FileDataLayer {
    cache: Vec< FdoStoredObject >
}

impl FileDataLayer {

    pub fn new() -> FileDataLayer {
        FileDataLayer {
            cache: Vec::new()
        }
    }

    pub fn new_user( &mut self, user: User ) {
        self.cache.push(
            FdoStoredObject::User( user )
        );
    }


}

thread_local! {
    pub static FDO: RefCell< FileDataLayer > = RefCell::new( FileDataLayer::new() )
}
