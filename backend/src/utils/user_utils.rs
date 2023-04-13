use crate::{
    config::db::Pool,
    models::user::User,
    utils::token_utils,
};
use actix_web::{
    http::header::HeaderValue,
    web
};

pub fn get_user_from_token(auth_header: Option<&HeaderValue>,  pool: &web::Data<Pool>) -> Option<User> {
    if let Some(header_value) = auth_header {
        if let Ok(authen_str) = header_value.to_str() {
            if authen_str.starts_with("bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                    if let Ok(username) = token_utils::verify_token(&token_data, pool) {
                        if let Ok(user) = User::find_user_by_username(&username, &pool.get().unwrap()) {
                            return Some(user);
                        }
                    }
                }
            }
        }
    }
    None
}