use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// config struct to manager user-defined config
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Config {
    proxy: Option<String>,
    model: Option<String>,
    api_key: String,
}

impl Config {
    fn empty_config() -> Self {
        Config {
            proxy: None,
            model: Some("gpt-3.5-turbo".into()),
            api_key: String::new(),
        }
    }
    pub fn read_from_file() -> Self {
        let path =
            PathBuf::from(format!("{}/.aibox", std::env::var("HOME").unwrap())).join("config.json");

        if path.exists() {
            let config = std::fs::read_to_string(path).unwrap();
            println!("read config from file: {:?}", config);
            serde_json::from_str(&config).unwrap()
        } else {
            let config = Config::empty_config();
            config.write_to_file();
            config
        }
    }
    pub fn write_to_file(&self) {
        let path =
            PathBuf::from(format!("{}/.aibox", std::env::var("HOME").unwrap())).join("config.json");

        if !path.exists() {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        }
        let config = serde_json::to_string(self).unwrap();
        println!("write config to file: {:?}, {:?}", config, path);
        std::fs::write(path, config).unwrap();
    }
    pub fn proxy_addr(&self) -> Option<&str> {
        self.proxy.as_deref()
    }
    pub fn api_key(&self) -> &str {
        self.api_key.as_str()
    }

    #[cfg(test)]
    pub fn test_new() -> Self {
        Config {
            proxy: Some("socks5h://127.0.0.1:1080".into()),
            model: Some("gpt-3.5-turbo".into()),
            api_key: "sk-xx".into(),
        }
    }
}
