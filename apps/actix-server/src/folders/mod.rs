use crate::prelude::*;
use yatir_folders::prelude::*;

use actix_web::{
    get, post,
    web::{self, ServiceConfig},
};

#[get("/{id}")]
async fn get_by_id(
    web::Path(id): web::Path<Index>,
    data: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let data = data.into_inner();
    let repo = data.folders_repo().ok_or(CoreError::internal_error())?;
    let folder = repo.get(id).await?;
    Ok(HttpResponse::Ok().json(folder))
}

#[get("/{id}/childs")]
async fn get_childs_by_id(
    web::Path(id): web::Path<Index>,
    data: web::Data<AppState>,
) -> ActixResult<HttpResponse> {
    let data = data.into_inner();
    let repo = data.folders_repo().ok_or(CoreError::internal_error())?;
    let folders = repo.get_children(id).await?;
    Ok(HttpResponse::Ok().json(folders))
}

#[post("/")]
async fn create(
    data: web::Data<AppState>,
    folder: web::Json<CreateFolder>,
) -> ActixResult<HttpResponse> {
    let folder = folder.into_inner();
    let data = data.into_inner();
    let repo = data.folders_repo().ok_or(CoreError::internal_error())?;
    let folder = repo.create(folder).await?;
    Ok(HttpResponse::Ok().json(folder))
}

pub fn setup_service(cfg: &mut ServiceConfig) {
    cfg.service(get_by_id)
        .service(get_childs_by_id)
        .service(create);
}
