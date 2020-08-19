use std::fs::read_to_string;

use actix_files::Files;
use actix_web::{Result, web, middleware};
use serde::{Deserialize, de::DeserializeOwned};
use actix_web::middleware::DefaultHeaders;

#[derive(Deserialize, Clone, Debug)]
pub struct StaticMap {
    path: String,
    dir: String,
    allow_listing: Option<bool>
}

#[derive(Deserialize, Clone, Debug)]
pub struct MySql {
    pub url: String
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub listen: String,
    pub static_map: Vec<StaticMap>,
    pub mysql: MySql
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
pub fn config_static_dir(cfg: &mut web::ServiceConfig, static_map: &Vec<StaticMap>) {
    for sm in static_map {
        let f = Files::new(sm.path.as_str(), sm.dir.clone())
            .use_etag(true);

        let allow_listing = match sm.allow_listing {
            Some(v) => v,
            None => false
        };

        cfg.service(if allow_listing { f.show_files_listing() } else {f});
    }
}

pub fn scaffolding_wrap() -> DefaultHeaders {
    middleware::DefaultHeaders::new().header("Server", "Actix")
}
