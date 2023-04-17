use crate::{
    models::{
        comments::CommentDTO,
        response::ResponseBody
    }, 
    config::db::Pool,
    constants,
    services::comments_service
};

use actix_web::{web, HttpRequest, HttpResponse, Result};



// POST api/schematic/{id}/comments
pub async fn post(
    comment_dto: web::Json<CommentDTO>,
    id: web::Path<String>,
    req: HttpRequest,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        comments_service::post(comment_dto.0,  id.into_inner(), authen_header, &pool);
        Ok(HttpResponse::Ok().json(ResponseBody::new(
            constants::MESSAGE_OK,
            constants::EMPTY,
        )))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(
            constants::MESSAGE_TOKEN_MISSING,
            constants::EMPTY,
        )))
    }
}
