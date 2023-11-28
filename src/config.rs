use std::{fs, str::FromStr};
use toml::Table;

// a simple struct representing the configuration.
pub struct Config {
    pub text: String
}

const CONFIG_PATH_NAME: &str = "typer.toml";
const DEFAULT_TEXT: &str = "hello world, this is a sample text. want your own? create a config file!";

// read the toml config. might fail if not readable for some reason
fn read_toml() -> Result<Config, ()>{
    let file_contnet = fs::read_to_string(CONFIG_PATH_NAME).map_err(|_|())?;
    let toml = Table::from_str(&file_contnet).map_err(|_|())?;

    let text = toml.get("text").ok_or(())?.as_str().ok_or(())?;
    Ok(Config{ text: text.to_string() })
}

// check and fix the config if it has any textual errors
fn check_and_fix_text(config_text: &mut String) {
    *config_text = config_text.trim().to_string();
}

// load the config
pub fn load_config() -> Config {
    let conf = read_toml();
    match conf {
        Ok(mut ex) => {
            check_and_fix_text(&mut ex.text);
            ex
        },
        Err(()) => Config { text: DEFAULT_TEXT.to_string() },
    }
}
