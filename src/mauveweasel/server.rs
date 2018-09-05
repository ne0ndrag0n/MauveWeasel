use mauveweasel::settings::Settings;

pub struct Server {
    settings: Settings
}

impl Server {
    pub fn new() -> Server {
        Server {
            settings: Settings::new()
        }
    }
}
