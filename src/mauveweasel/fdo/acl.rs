use mauveweasel::fdo::user::User;

pub struct UrlAcl<'a> {
    list: Vec< ( &'a User, String ) >
}

#[derive(Serialize,Deserialize)]
pub struct StorableUrlAcl {
    list: Vec< ( String, String ) >
}
