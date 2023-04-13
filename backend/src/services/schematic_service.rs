use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::schematic::{SchematicDTO, Schematic},
    utils::user_utils
};
use actix_web::{
    http::{header::HeaderValue, StatusCode},
    web,
};

pub fn upload(schematic: SchematicDTO, authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<String, ServiceError>  {
    let user = match user_utils::get_user_from_token(Some(authen_header), pool) {
        Some(user) => user,
        None => {
            return Err(ServiceError::new(
                StatusCode::UNAUTHORIZED,
                constants::MESSAGE_UNAUTHORIZED.to_string(),
            ));
        }
    };

    match Schematic::upload(schematic, user,&pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::BAD_REQUEST, message)),
    }
}

pub fn delete(post_id: String, authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    !unimplemented!()
}

pub fn update(schematic: SchematicDTO, authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    !unimplemented!()
} 