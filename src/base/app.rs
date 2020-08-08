use std::fs::read_to_string;
use std::process::exit;

use serde::Deserialize;
use toml::de::Error;

#[derive(Deserialize)]
pub struct AppConfig {
    pub listen: String
}

/// 读取 res/config/app.toml 中的配置
pub fn get_app_config() -> AppConfig {
    let read = read_to_string("res/config/app.toml");
    match read {
        Ok(text) => {
            let toml: Result<AppConfig, Error> = toml::from_str(text.as_str());
            match toml {
                Ok(v) => v,
                Err(e) => {
                    println!("配置解析失败： {}", e.to_string());
                    exit(1);
                }
            }
        },
        Err(e) => {
            println!("读取配置文件失败： {}", e.to_string());
            exit(1);
        }
    }
}
