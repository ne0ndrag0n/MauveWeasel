#[derive(Serialize,Deserialize)]
pub struct Settings {
    port: u16
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            port: 3001
        }
    }

    pub fn port( &self ) -> u16 {
        self.port
    }
}
