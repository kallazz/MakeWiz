//! Manages the user config file.
//!
//! The user config file, stored in TOML format, holds default values for the C/C++ compiler
//! and executable name. Users can set these values to avoid specifying them every time they
//! use MakeWiz. If no custom defaults are chosen, they will be set to:
//!
//! - Compiler: g++
//! - Executable name: main
//!
//! This module provides functions for reading, updating, and printing the user configuration.

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;

/// Represents an attribute that can be updated in the user config file.
///
/// These attributes mirror the field names in the config file.
pub enum Attribute {
    CompilerName(String),
    ExecutableName(String),
}

/// Represents the user config file.
/// 
/// The user config file is stored in TOML format and can be used to set default values
/// to avoid specifying them each time MakeWiz is used.
#[derive(Serialize, Deserialize, PartialEq, Debug)] 
pub struct UserConfig {
    pub compiler_name: String,
    pub executable_name: String,
}

impl UserConfig {
    /// Retrieves the current config file from the specified file path.
    /// If the file doesn't exist, it creates it with default values(compiler: g++, executable: main).
    ///
    /// # Arguments
    ///
    /// * `config_path` - The path to the config file.
    ///
    /// # Returns
    ///
    /// A `UserConfig` instance containing the current configuration.
    pub fn get_current_config(config_path: &Path) -> UserConfig {
        if !Path::new(config_path).exists() { UserConfig::create_config_file(UserConfig::default(), config_path); }

        let mut file = std::fs::File::open(config_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let config: UserConfig = toml::from_str(&contents).unwrap();

        config
    }

    fn default() -> Self {
        Self {
            compiler_name: String::from("g++"),
            executable_name: String::from("main"),
        }
    }

    /// Updates the config file with the provided attribute.
    /// This can be used to change the default compiler name or executable name.
    ///
    /// # Arguments
    ///
    /// * `attribute` - The field to update.
    /// * `config_path` - The path to the config file.
    pub fn update_config(attribute: Attribute, config_path: &Path) {
        let mut config = UserConfig::get_current_config(config_path);

        match attribute {
            Attribute::CompilerName(name) => { config.compiler_name = name; }
            Attribute::ExecutableName(name) => { config.executable_name = name; }
        }

        UserConfig::create_config_file(config, config_path);
    }

    fn create_config_file(config: UserConfig, config_path: &Path) {
        let serialized_config = toml::to_string(&config).unwrap();

        let mut file = File::create(config_path).unwrap();
        file.write_all(serialized_config.as_bytes()).unwrap();
    }

    /// Prints the current values of the config file.
    ///
    /// # Arguments
    ///
    /// * `config_path` - The path to the config file.
    pub fn print_config_values(config_path: &Path) {
        let config = UserConfig::get_current_config(config_path);

        println!("Default compiler name: {}", config.compiler_name);
        println!("Default executable name: {}", config.executable_name);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn getting_config() {
        let config_path = Path::new("./test-dirs/test-config/config-to-get/config.toml");

        let expected = UserConfig {
            compiler_name: String::from("compiler name to get"),
            executable_name: String::from("executable name to get"),
        };

        assert_eq!(expected, UserConfig::get_current_config(config_path));
    }

    #[test]
    fn config_not_created() {
        let config_path = Path::new("./test-dirs/test-config/config-not-created/config.toml");

        let expected = UserConfig::default();

        assert_eq!(expected, UserConfig::get_current_config(config_path));

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn config_created() {
        let config_path = Path::new("./test-dirs/test-config/config-created/config.toml");

        let created_config = UserConfig {
            compiler_name: String::from("created compiler name"),
            executable_name: String::from("created executable name"),
        };
        UserConfig::create_config_file(created_config, config_path);

        let expected = UserConfig {
            compiler_name: String::from("created compiler name"),
            executable_name: String::from("created executable name"),
        };

        assert_eq!(expected, UserConfig::get_current_config(config_path));
    }

    #[test]
    fn updating_config() {
        let config_path = Path::new("./test-dirs/test-config/config-to-update/config.toml");

        let created_config = UserConfig {
            compiler_name: String::from("created compiler name"),
            executable_name: String::from("created executable name"),
        };
        UserConfig::create_config_file(created_config, config_path);

        UserConfig::update_config(Attribute::CompilerName(String::from("new compiler name")), config_path);

        let expected = UserConfig {
            compiler_name: String::from("new compiler name"),
            executable_name: String::from("created executable name"),
        };

        assert_eq!(expected, UserConfig::get_current_config(config_path));

        UserConfig::update_config(Attribute::ExecutableName(String::from("new executable name")), config_path);

        let expected = UserConfig {
            compiler_name: String::from("new compiler name"),
            executable_name: String::from("new executable name"),
        };

        assert_eq!(expected, UserConfig::get_current_config(config_path));
    }
}
