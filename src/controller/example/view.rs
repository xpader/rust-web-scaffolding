use actix_web::{web, Responder, get};
use tera::{Tera, Context};
use chrono::prelude::{DateTime, Local};

use crate::base::template::render;

/// 使用 tera 引擎的视图
#[get("/view")]
pub async fn view(tmpl: web::Data<Tera>) -> impl Responder {
    //往视图中加入要渲染的值
    let mut context = Context::new();

    //当前时间
    let local: DateTime<Local> = Local::now();
    let f_now = local.format("%Y-%m-%d %H:%M:%S").to_string();
    context.insert("now", &f_now);

    render(&tmpl, "index.html", &context)
}