use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;

pub enum Attribute {
    CompilerName(String),
    ExecutableName(String),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)] 
pub struct Config {
    pub compiler_name: String,
    pub executable_name: String,
}

impl Config {
    fn default() -> Self {
        Self {
            compiler_name: String::from("g++"),
            executable_name: String::from("main"),
        }
    }

    pub fn init_config(config_path: &Path) {
        if !Path::new(config_path).exists() { Config::create_config(Config::default(), config_path); }
    }

    pub fn update_config(attribute: Attribute, config_path: &Path) {
        let mut config = Config::get_current_config(config_path);

        match attribute {
            Attribute::CompilerName(name) => { config.compiler_name = name; }
            Attribute::ExecutableName(name) => { config.executable_name = name; }
        }

        Config::create_config(config, config_path);
    }

    pub fn print_config_values(config_path: &Path) {
        let config = Config::get_current_config(config_path);

        println!("Default compiler name: {}", config.compiler_name);
        println!("Default executable name: {}", config.executable_name);
    }

    pub fn get_current_config(config_path: &Path) -> Config {
        let mut file = std::fs::File::open(config_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let config: Config = toml::from_str(&contents).unwrap();

        config
    }

    fn create_config(config: Config, config_path: &Path) {
        let serialized_config = toml::to_string(&config).unwrap();

        let mut file = File::create(config_path).unwrap();
        file.write_all(serialized_config.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn getting_config() {
        let config_path = Path::new("./test-dirs/test-config/config-to-get/config.toml");

        let expected = Config {
            compiler_name: String::from("compiler name to get"),
            executable_name: String::from("executable name to get"),
        };

        assert_eq!(expected, Config::get_current_config(config_path));
    }

    #[test]
    fn init_config_not_created() {
        let config_path = Path::new("./test-dirs/test-config/config-not-created/config.toml");
        Config::init_config(config_path);

        let expected = Config::default();

        assert_eq!(expected, Config::get_current_config(config_path));

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn init_config_created() {
        let config_path = Path::new("./test-dirs/test-config/config-created/config.toml");

        let created_config = Config {
            compiler_name: String::from("created compiler name"),
            executable_name: String::from("created executable name"),
        };
        Config::create_config(created_config, config_path);

        Config::init_config(config_path);

        let expected = Config {
            compiler_name: String::from("created compiler name"),
            executable_name: String::from("created executable name"),
        };

        assert_eq!(expected, Config::get_current_config(config_path));
    }

    #[test]
    fn updating_config() {
        let config_path = Path::new("./test-dirs/test-config/config-to-update/config.toml");

        let created_config = Config {
            compiler_name: String::from("created compiler name"),
            executable_name: String::from("created executable name"),
        };
        Config::create_config(created_config, config_path);

        Config::update_config(Attribute::CompilerName(String::from("new compiler name")), config_path);

        let expected = Config {
            compiler_name: String::from("new compiler name"),
            executable_name: String::from("created executable name"),
        };

        assert_eq!(expected, Config::get_current_config(config_path));

        Config::update_config(Attribute::ExecutableName(String::from("new executable name")), config_path);

        let expected = Config {
            compiler_name: String::from("new compiler name"),
            executable_name: String::from("new executable name"),
        };

        assert_eq!(expected, Config::get_current_config(config_path));
    }
}
