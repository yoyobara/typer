use std::{fs, error::Error};
use rand::seq::SliceRandom;
use regex::Regex;
use toml::Table;

const CONFIG_PATH_NAME: &str = "typer.toml";
const DEFAULT_TEXT: &str = "hello world, this is a sample text. want your own? create a config file!";

// a simple struct representing the configuration.
pub struct Config {
    pub text: String
}

// check and fix the config if it has any textual errors
fn check_and_fix_text(config_text: &mut String) {
    let pattern: Regex = Regex::new(r"\s+").unwrap();

    *config_text = pattern.replace_all(&config_text, " ").trim().to_string();
}

fn text_from_language(words: &Vec<toml::Value>, length: i64) -> Result<String, Box<dyn Error>>{

    let mut text_words: Vec<&str> = Vec::new();

    for _ in 0..length {
        let word = words.choose(&mut rand::thread_rng()).unwrap().as_str().ok_or("the language array contains a non string")?;
        text_words.push(word);
    }

    Ok(text_words.join(" "))
}

// load the config
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    let toml_file_open = fs::read_to_string(CONFIG_PATH_NAME);

    match toml_file_open {
        Ok(s) => {
            let toml_config: Table = toml::from_str(&s)?;
            let method = toml_config
                .get("method").ok_or("no 'method' key")?
                .as_str().ok_or("no String 'method' key")?;

            match method {
                "text" => {
                    let mut text_from_config = toml_config
                        .get("text").ok_or("no text specified")?
                        .as_str().ok_or("text is not String")?
                        .to_string();

                    check_and_fix_text(&mut text_from_config);

                    Ok(Config { text: text_from_config })
                }
                "language" => {
                    let words_arr = toml_config.
                        get("language").ok_or("no language specified")?
                        .as_array().ok_or("language is not an array of words")?;

                    let length = toml_config.
                        get("length").ok_or("no length specified")?
                        .as_integer().ok_or("length is not integer")?;

                    let text = text_from_language(words_arr, length)?;

                    Ok(Config{ text })
                }
                _ => Err("method should be `language` or `text`".into())
            }
        }
        ,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Ok(Config { text: DEFAULT_TEXT.to_string() }),
            _ => Err(e.into())
        }
    }
}
