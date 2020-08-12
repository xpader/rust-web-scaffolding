use actix_web::{get, Responder, web};

use crate::base::app::AppConfig;

#[get("/state/config")]
pub async fn state(config: web::Data<AppConfig>) -> impl Responder {
    format!("Current config {:?}.", config)
}