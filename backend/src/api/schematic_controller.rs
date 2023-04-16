use crate::{
    config::db::Pool,
    constants,
    models::{
        response::ResponseBody,
        schematic::{Schematic, SchematicDTO},
        filters::SchematicFilter,
    },
    services::schematic_service,
};
use actix_web::{web, HttpResponse, HttpRequest, Result};

// POST api/schematic/upload
pub async fn upload(
    schematic_dto: web::Json<SchematicDTO>,
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        schematic_service::upload(schematic_dto.0, authen_header, &pool);
        Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_UPLOAD_SUCCESS,
            constants::EMPTY,
        )))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(
            constants::MESSAGE_TOKEN_MISSING,
            constants::EMPTY,
        )))
    }
}

// GET api/schematic/search
pub async fn search(
    web::Query(filter): web::Query<SchematicFilter>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match schematic_service::search(filter, &pool) {
        Ok(page) => Ok(HttpResponse::Ok().json(page)),
        Err(err) => Ok(err.response()),
    }
}