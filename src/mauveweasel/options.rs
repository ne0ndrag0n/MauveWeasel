#[derive(Serialize,Deserialize)]
pub struct Config {
    ip: Option< String >,
    port: Option< u16 >,
    postbox_directory: Option< String >,
    max_request_size: Option< u64 >,
    templates_directory: Option< String >,
    reverse_proxy_prefix: Option< String >,
    cookiejar_directory: Option< String >,
    newsgen_directory: Option< String >,
    newsgen_output_directory: Option< String >,
    newsgen_toc_filename: Option< String >
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
        match self.templates_directory {
            Some( ref val ) => &val,
            None => "./templates"
        }
    }

    pub fn reverse_proxy_prefix( &self ) -> &str {
        match self.reverse_proxy_prefix {
            Some( ref val ) => &val,
            None => "/d"
        }
    }

    pub fn cookiejar_directory( &self ) -> &str {
        match self.cookiejar_directory {
            Some( ref val ) => &val,
            None => "./cookiejar"
        }
    }

    pub fn newsgen_directory( &self ) -> &str {
        match self.newsgen_directory {
            Some( ref val ) => &val,
            None => "./newsgen"
        }
    }

    pub fn newsgen_output_directory( &self ) -> &str {
        match self.newsgen_output_directory {
            Some( ref val ) => &val,
            None => "./news"
        }
    }

    pub fn newsgen_toc_filename( &self ) -> &str {
        match self.newsgen_toc_filename {
            Some( ref val ) => &val,
            None => "toc.html"
        }
    }

    pub fn get_host( &self ) -> String {
        self.ip().to_string() + ":" + self.port().to_string().as_str()
    }

}
