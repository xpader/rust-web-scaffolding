use actix_web::{get, Responder, web};

use crate::AppState;

#[get("/state/config")]
pub async fn state(state: web::Data<AppState>) -> impl Responder {
    format!("Current config {:?}.", state.config)
}