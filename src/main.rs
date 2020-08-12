use actix_web::{App, HttpServer, middleware};

mod base;
mod controller;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //读取程序基础配置
    let config = base::app::get_app_config();
    let config_clone = config.clone();

    println!("Start listen http://{}", config.listen);

    HttpServer::new(move || {
        let config = config_clone.clone();
        App::new()
            .data(config)
            .wrap(middleware::DefaultHeaders::new().header("Server", "Actix"))
            .configure(controller::config_routes)
            .configure(base::app::config_static_dir)
            .configure(base::view::config_tmpl_engine)
    })
        .bind(&config.listen)?
        .run()
        .await
}

