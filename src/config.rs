use std::error::Error;

use toml::Value;

// TODO: not efficient but good enough
pub(crate) fn get_config_value(key: &str) -> Result<Value, Box<dyn Error>> {
    let root_path = std::env::current_dir()?;

    let config_path = root_path.join("config.toml");

    let config = std::fs::read_to_string(config_path)?;

    let config: toml::Value = toml::from_str(&config)?;

    let value = config
        .get(key)
        .expect(&format!("Could not find key {} in config file", key));

    Ok(value.clone())
}
