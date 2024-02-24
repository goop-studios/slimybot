use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct Welcome {
    pub enabled: bool,
    pub channel: Option<u64>
}

#[derive(Debug, Default)]
pub struct AutoRole {
    pub enabled: bool,
    pub role: Option<u64>
}

#[derive(Debug)]
pub struct Data {
    pub config_dir: Mutex<String>,
    // Section Welcome
    pub welcome: Mutex<Welcome>,
    // Section AutoRole
    pub autorole: Mutex<AutoRole>
    
} // User data, which is stored and accessible in all command invocations

impl Default for Data {
    fn default() -> Self {
        Data {
            config_dir: Mutex::new("config.toml".to_string()),
            welcome: Mutex::new(Welcome::default()),
            autorole: Mutex::new(AutoRole::default())
        }
    }
}
