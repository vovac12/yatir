use crate::prelude::*;

use actix_web::{get, web::ServiceConfig};

#[get("/")]
async fn get_by_id() -> ActixResult<HttpResponse> {
    Err(CoreError::not_implemented("Users not implemented"))?
}

pub fn setup_service(cfg: &mut ServiceConfig) {
    cfg.service(get_by_id);
}
