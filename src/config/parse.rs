use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    // Sections
    pub welcome: Welcome,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    // Fields
    pub enabled: bool,
    pub channel: u64,
}

pub fn parse_config() -> Result<BotConfig, Box<dyn std::error::Error>> {
    let config = std::fs::read_to_string("config.toml")?;
    let config: BotConfig = toml::from_str(&config)?;
    println!("{:?}", config);
    Ok(config)
}
