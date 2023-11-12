use std::path::Path;

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
    }
}
