use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        schematic::{
            SchematicDTO, 
            Schematic
        },
        filters::SchematicFilter, response::Page
    },
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

pub fn delete(post_id: String, authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    let user = match user_utils::get_user_from_token(Some(authen_header), pool) {
        Some(user) => user,
        None => {
            return Err(ServiceError::new(
                StatusCode::UNAUTHORIZED,
                constants::MESSAGE_UNAUTHORIZED.to_string(),
            ));
        }
    };

    match Schematic::find_by_id(post_id.clone(),&pool.get().unwrap()) {
        Ok(Some(schematic)) => {
            if schematic.author != user.id {
                return Err(ServiceError::new(
                    StatusCode::UNAUTHORIZED,
                    constants::MESSAGE_UNAUTHORIZED.to_string(),
                ));
            }
            match Schematic::delete(post_id, &pool.get().unwrap()) {  
                Ok(_) => Ok(()),
                Err(_) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
                )),
            }
        },
        _ => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Schematic with id {} not found", post_id),
        )),
    }
}

pub fn update(
    post_id: String, 
    authen_header: &HeaderValue, 
    updated_schematic: SchematicDTO,
    pool: &web::Data<Pool>
) -> Result<(), ServiceError> {
    let user = match user_utils::get_user_from_token(Some(authen_header), pool) {
        Some(user) => user,
        None => {
            return Err(ServiceError::new(
                StatusCode::UNAUTHORIZED,
                constants::MESSAGE_UNAUTHORIZED.to_string(),
            ));
        }
    };

    match Schematic::find_by_id(post_id.clone(),&pool.get().unwrap()) {
        Ok(Some(schematic)) => {
            if schematic.author != user.id {
                return Err(ServiceError::new(
                    StatusCode::UNAUTHORIZED,
                    constants::MESSAGE_UNAUTHORIZED.to_string(),
                ));
            }
            match Schematic::update(post_id,  updated_schematic,&pool.get().unwrap()) {  
                Ok(_) => Ok(()),
                Err(_) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
                )),
            }
        },
        _ => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Schematic with id {} not found", post_id),
        )),
    }
}

pub fn get(post_id: String, pool: &web::Data<Pool>) -> Result<Schematic, ServiceError> {
    match Schematic::find_by_id(post_id,&pool.get().unwrap()) {
        Ok(Some(schematic)) => Ok(schematic),
        _ => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            constants::MESSAGE_SCHEMATIC_NOT_FOUND.to_string()
        ))
    }
}

pub fn search(filter: SchematicFilter, pool: &web::Data<Pool>) -> Result<Page<Schematic>, ServiceError> {
    match Schematic::filter(filter, &pool.get().unwrap()) {
        Ok(schematics) => Ok(schematics),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        ))
    }
}