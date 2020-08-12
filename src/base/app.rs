use std::fs::read_to_string;
use std::process::exit;
use std::path::PathBuf;

use serde::Deserialize;
use toml::de::Error;

use actix_files::NamedFile;
use actix_web::{HttpRequest, Result, get};

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

/// 静态文件控制器
///
/// 修改及控制静态文件路径及具体所在目录
///
/// 具体参考：
/// - https://actix.rs/docs/static-files/
#[get("/{filename:.*}")]
pub async fn static_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    println!("Static: {:?}", path);
    Ok(NamedFile::open(path)?)
}
