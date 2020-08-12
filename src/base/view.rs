use actix_web::{HttpResponse, Responder, web};
use tera::{Context, Tera};

/// 初始化 tera 模板引擎
pub fn config_tmpl_engine(cfg: &mut web::ServiceConfig) {
    match Tera::new("res/template/**/*.html") {
        Ok(t) => cfg.data(t),
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
}

/// 渲染视图
pub fn render(tmpl: &Tera, page: &str, context: &Context) -> impl Responder {
    let output = tmpl.render(page, &context).unwrap();
    HttpResponse::Ok().set_header("Content-Type", "text/html; charset=utf-8").body(output)
}
