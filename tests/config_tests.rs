use genmake::defaults::Config;

use std::env;
use std::path::PathBuf;
use std::fs;

#[test]
fn all_config_testing() {
    let testing_dir = PathBuf::from("./test-dirs/test-config/");
    env::set_current_dir(&testing_dir).unwrap();

    //Set default config
    Config::init_config();

    let config = Config::get_current_config();

    let expected_config = Config {
        compiler_name: "g++".to_string(),
        executable_name: "main".to_string()
    };

    assert_eq!(expected_config, config);

    //Update default config
    let new_config = Config {
        executable_name: "new executable name".to_string(),
        compiler_name: "new compiler name".to_string(),
    };

    Config::update_compiler(&new_config.compiler_name);
    Config::update_executable(&new_config.executable_name);

    assert_eq!(new_config, Config::get_current_config());

    fs::remove_file("config.toml").unwrap();
}