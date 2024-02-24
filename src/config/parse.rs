use std::{
    fs::{
        read_to_string,
        write
    }, path::Path
};
use serde::{Deserialize, Serialize};

type Error = Box<dyn std::error::Error>;


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BotConfig {
    // Sections
    pub welcome: Welcome,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Welcome {
    // Fields
    pub enabled: bool,
    pub channel: u64,
}

impl BotConfig {
    pub fn read(file: &Path) -> Result<Self, Error> {
        let file_contents = read_to_string(file)?;
        let read_config: BotConfig = toml::from_str(&file_contents)?;
        Ok(read_config)
    }

    pub fn write(self, file: &Path) -> Result<(), Error> {
        let tomlized_config = toml::to_string_pretty(&self)?;
        write(file, tomlized_config)?;
        Ok(())
    }
}
