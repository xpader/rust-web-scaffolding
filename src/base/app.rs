use std::fs::read_to_string;

use actix_files::Files;
use actix_web::{Result, web};
use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub listen: String
}

/// 读取配置文件
///
/// 读取来自 res/config/app.toml 的配置文件
pub fn get_app_config() -> AppConfig {
    read_config::<AppConfig>("app.toml").unwrap()
}

pub fn read_config<T>(filename: &str) -> Result<T, String>
where
    T: DeserializeOwned
{
    let tr = read_to_string(format!("res/config/{}", filename));
    match tr {
        Ok(text) => {
            let p = text.as_str();
            let tt: Result<T, toml::de::Error> = toml::from_str(p);
            match tt {
                Ok(t) => Ok(t),
                Err(e) => Err(e.to_string())
            }
        },
        Err(e) => Err(e.to_string())
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
