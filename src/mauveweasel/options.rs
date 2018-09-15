#[derive(Serialize,Deserialize)]
pub struct Config {
    ip: Option< String >,
    port: Option< u16 >,
    postbox_directory: Option< String >,
    max_request_size: Option< u64 >,
    templates_directory: Option< String >
}

impl Config {

    pub fn ip( &self ) -> &str {
        match self.ip {
            Some( ref val ) => &val,
            None => "127.0.0.1"
        }
    }

    pub fn port( &self ) -> u16 {
        match self.port {
            Some( val ) => val,
            None => 3000
        }
    }

    pub fn postbox_directory( &self ) -> &str {
        match self.postbox_directory {
            Some( ref val ) => &val,
            None => "./postbox"
        }
    }

    pub fn max_request_size( &self ) -> u64 {
        match self.max_request_size {
            Some( val ) => val,
            None => 4096
        }
    }

    pub fn templates_directory( &self ) -> &str {
        match self.postbox_directory {
            Some( ref val ) => &val,
            None => "./templates"
        }
    }

    pub fn get_host( &self ) -> String {
        self.ip().to_string() + ":" + self.port().to_string().as_str()
    }

}
