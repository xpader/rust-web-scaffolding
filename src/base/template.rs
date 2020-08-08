use actix_web::{HttpResponse, Responder};
use tera::{Context, Tera};

pub fn render(tmpl: &Tera, page: &str, context: &Context) -> impl Responder {
    let output = tmpl.render(page, &context).unwrap();
    HttpResponse::Ok().set_header("Content-Type", "text/html; charset=utf-8").body(output)
}
