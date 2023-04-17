use crate::{
    models::comments::{CommentDTO, Comment},
    config::db::Pool,
    error::ServiceError, utils::user_utils, constants,
};

use actix_web::{
    http::{HeaderValue, StatusCode}, 
    web
};

pub fn post(comment: CommentDTO, schematic_id: String, authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match user_utils::get_user_from_token(Some(authen_header), pool) {
        Some(user) => {
            match Comment::post(comment,  schematic_id, user.id, &pool.get().unwrap()) {  
                Ok(message) => Ok(message),
                Err(message) => Err(ServiceError::new(StatusCode::BAD_REQUEST, message)),
            }
        }
        None => {
            Err(ServiceError::new(
                StatusCode::UNAUTHORIZED,
                constants::MESSAGE_UNAUTHORIZED.to_string(),
            ))
        }
    }
}

pub fn delete(comment_id: String, authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match user_utils::get_user_from_token(Some(authen_header), pool) {
        Some(user) => {
            match Comment::delete(comment_id, user,&pool.get().unwrap()) {
                Ok(_) => Ok(()),
                Err(e) => Err(ServiceError::new(StatusCode::BAD_REQUEST,e))
            }
        }
        None => {
            Err(ServiceError::new(
                StatusCode::UNAUTHORIZED,
                constants::MESSAGE_UNAUTHORIZED.to_string(),
            ))
        }
    }
}

pub fn update(comment_id: String, comment: CommentDTO, authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match user_utils::get_user_from_token(Some(authen_header), pool) {
        Some(user) => {
            match Comment::update(comment_id, user, comment, &pool.get().unwrap()) {
                Ok(_) => Ok(()),
                Err(e) => Err(ServiceError::new(StatusCode::BAD_REQUEST,e))
            }
        }
        None => {
            Err(ServiceError::new(
                StatusCode::UNAUTHORIZED,
                constants::MESSAGE_UNAUTHORIZED.to_string(),
            ))
        }
    }
}