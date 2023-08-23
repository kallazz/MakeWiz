use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{Write, Read};

#[derive(Serialize, Deserialize, PartialEq, Debug)] 
pub struct Config {
    pub compiler_name: String,
    pub executable_name: String,
}

impl Config {
    fn set_new_config(config: Config) {
        let serialized_config = toml::to_string(&config).unwrap();

        let mut file = File::create("config.toml").unwrap();
        file.write_all(serialized_config.as_bytes()).unwrap();
    }

    fn create_default_config() {
        let config = Config {
            compiler_name: "g++".to_string(),
            executable_name: "main".to_string(),
        };

        let serialized_config = toml::to_string(&config).unwrap();

        let mut file = File::create("config.toml").unwrap();
        file.write_all(serialized_config.as_bytes()).unwrap();
    }

    pub fn init_config() {
        let file_path = "config.toml";

        match fs::metadata(file_path) {
            Ok(metadata) => {
                if metadata.is_file() { }
                else {
                    Config::create_default_config();
                }
            }
            Err(_) => {
                Config::create_default_config();
            }
        }
    }

    pub fn get_current_config() -> Config {
        let mut file = std::fs::File::open("config.toml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let config: Config = toml::from_str(&contents).unwrap();

        config
    }

    pub fn update_compiler(compiler_name: &str) {
        let mut config = Config::get_current_config();

        config.compiler_name = compiler_name.to_string();

        Config::set_new_config(config);
    }

    pub fn update_executable(executable_name: &str) {
        let mut config = Config::get_current_config();

        config.executable_name = executable_name.to_string();

        Config::set_new_config(config);
    }

    pub fn get_current_compiler() -> String {
        let config = Config::get_current_config();

        config.compiler_name
    }

    pub fn get_current_executable() -> String {
        let config = Config::get_current_config();

        config.executable_name
    }
}
