use actix_web::{App, HttpServer, middleware, web};
use tera::Tera;

use crate::base::app::get_app_config;

mod base;
mod controller;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //读取程序基础配置
    let config = get_app_config();

    println!("Start listen http://{}", config.listen);

    HttpServer::new(|| {
        //初始化 tera 模板引擎
        let tera = get_tera();

        App::new()
            .data(tera)
            .wrap(middleware::DefaultHeaders::new().header("Server", "Actix"))
            .route("/", web::get().to(controller::index::hello))
            .route("/t/{name}", web::get().to(controller::index::hello))
            .service(controller::example::view::about)
            .service(controller::example::view::view)
    })
        .bind(&config.listen)?
        .run()
        .await
}

fn get_tera() -> Tera {
    match Tera::new("res/template/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    }
}