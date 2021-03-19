pub use crate::{error::*, state::*};
pub use actix_web::{
    http::{self, StatusCode},
    web::{self, HttpResponse, Json},
};
pub use yatir_core::prelude::*;
pub type ActixResult<T> = actix_web::Result<T, ActixError>;
