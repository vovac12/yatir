use crate::prelude::*;
use actix_web::{
    error::{JsonPayloadError, PathError},
    ResponseError,
};
use std::fmt;

pub fn configure_error_handlers(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::JsonConfig::default().error_handler(|err, _| ActixError::from(err).into()))
        .app_data(web::PathConfig::default().error_handler(|err, _| ActixError::from(err).into()));
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActixError {
    code: i64,
    message: String,
}

impl fmt::Display for ActixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl ResponseError for ActixError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> web::HttpResponse {
        web::HttpResponse::build(self.status_code()).json(self)
    }
}

impl From<CoreError> for ActixError {
    fn from(e: CoreError) -> Self {
        match e {
            CoreError::NotFound(msg) => Self {
                code: StatusCode::NOT_FOUND.as_u16().into(),
                message: msg,
            },
            CoreError::NotImplemented(msg) => Self {
                code: StatusCode::NOT_IMPLEMENTED.as_u16().into(),
                message: msg,
            },
            CoreError::InternalError => Self {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16().into(),
                message: StatusCode::INTERNAL_SERVER_ERROR.as_str().into(),
            },
        }
    }
}

impl From<JsonPayloadError> for ActixError {
    fn from(err: JsonPayloadError) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST.as_u16().into(),
            message: format!("Json: {}", err),
        }
    }
}

impl From<PathError> for ActixError {
    fn from(err: PathError) -> Self {
        Self {
            code: StatusCode::NOT_FOUND.as_u16().into(),
            message: format!("Path: {}", err),
        }
    }
}
