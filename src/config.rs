use std::{path::Path, fs::File, io::Read, str::FromStr};

use toml::Table;

const CONFIG_PATH_NAME: &str = "typer.toml";
const SAMPLE_TEXT: &str = "hello world, this is a sample text.\n want your own?\ncreate a config file!";

// a simple struct representing the configuration.
struct Config {
    text: String,
}

// load the config
fn load_config() -> Config {
    let config_path = Path::new(CONFIG_PATH_NAME);

    if config_path.exists() {

        // get file data
        let mut file_data = String::new();
        File::open(CONFIG_PATH_NAME).expect("could not open file").read_to_string(&mut file_data).expect("could not read file");

        // get config table
        let config = Table::from_str(&file_data).expect("error reading the toml config");

        // return matching config
        Config{text: config.get("text").expect("could not get the 'text' key").as_str().expect("text value is not a string..").to_string() }
    }
    else {
        Config { text: SAMPLE_TEXT.to_string() }
    }
}
