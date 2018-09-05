#[derive(Serialize,Deserialize)]
pub struct Settings {
    working_dir: String,
    port: u16
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            working_dir: ".".to_string(),
            port: 3001
        }
    }

    pub fn working_dir( &self ) -> &String {
        &self.working_dir
    }

    pub fn port( &self ) -> u16 {
        self.port
    }
}
