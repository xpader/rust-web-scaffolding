use actix_web::{web};

mod index;
mod example;

/// 系统路由配置
///
/// 在此统一配置系统路由与控制器关系，避免修改 main
/// 支持 resource, route, service 等各项配置
///
/// 具体参考：
/// - https://actix.rs/docs/application/
/// - https://actix.rs/docs/url-dispatch/
pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::hello))
        .route("/t/{name}", web::get().to(index::hello))
        .service(example::view::about)
        .service(example::view::view)
        .service(example::state::state)
        .service(example::db::query)
        .service(example::db::view)
        .service(example::db::add)
        .service(example::db::add_post)
        .service(example::redis::getkey)
        .service(example::redis::getcache)
        ;
}