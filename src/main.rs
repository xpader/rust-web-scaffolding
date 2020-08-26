use std::sync::Arc;

use actix_web::{App, HttpServer, web::ServiceConfig};

mod base;
mod controller;

type AppConfig = Arc<base::app::AppConfig>;

pub struct AppState {
    pub config: AppConfig,
    pub db: base::db::DbPool,
    pub redis: deadpool_redis::Pool,
    pub cache: base::cache::Cache
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    //读取程序基础配置
    let app_config = base::app::get_app_config();
    let app_config = Arc::new(app_config);

    let app_config_own = Arc::clone(&app_config);
    println!("Start listen http://{}", app_config_own.listen);

    println!("Create db pool.");
    let pool = base::db::create_pool(&app_config.mysql.url).await;

    println!("Create redis pool.");
    let redis_pool = base::redis::create_pool(&app_config_own.redis);

    let cache = base::cache::Cache::new(redis_pool.clone());

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                config: app_config.clone(),
                db: pool.clone(),
                redis: redis_pool.clone(),
                cache: cache.clone()
            })
            .wrap(base::app::scaffolding_wrap())
            .configure(controller::config_routes)
            .configure(|cfg: &mut ServiceConfig| {
                let config = app_config.clone();
                base::app::config_static_dir(cfg, &config.static_map);
            })
            .configure(base::view::config_tmpl_engine)
    })
        .bind(&app_config_own.listen)?
        .run()
        .await
}

