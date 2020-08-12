use std::fs::read_to_string;
use std::process::exit;

use serde::Deserialize;
use toml::de::Error;

use actix_files::{Files};
use actix_web::{Result, web};

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub listen: String
}

/// 读取配置文件
///
/// 读取来自 res/config/app.toml 的配置文件
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

/// 静态文件控制器
///
/// 修改及控制静态文件路径及具体所在目录
///
/// 具体参考：
/// - https://actix.rs/docs/static-files/
pub fn config_static_dir(cfg: &mut web::ServiceConfig) {
    cfg.service(
        Files::new("/static", "static")
            .use_etag(true)
            // .show_files_listing()
    );
}
