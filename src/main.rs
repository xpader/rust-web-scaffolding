use actix_web::{App, get, HttpServer, middleware, Responder, web};
use tera::Tera;

mod base;
mod controller;

#[get("/about")]
async fn about() -> impl Responder {
    format!("This is <<About Me>> page.")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let bind: &str = "0.0.0.0:8082";

    println!("Start actix-web/2.0 on http://{}", bind);

    HttpServer::new(|| {
        //初始化 tera 模板引擎
        let tera = match Tera::new("res/template/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        App::new()
            .data(tera)
            .wrap(middleware::DefaultHeaders::new().header("Server", "actix-web/2.0"))
            .route("/", web::get().to(controller::index::hello))
            .route("/t/{name}", web::get().to(controller::index::hello))
            .service(about)
            .service(controller::example::view::view)
    })
        .bind(bind)?
        .run()
        .await
}