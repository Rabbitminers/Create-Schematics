use crate::api::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg.service(
        web::scope("/api")
            .service(ping_controller::ping)
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/signup").route(web::post().to(account_controller::signup)),
                    )
                    .service(
                        web::resource("/login").route(web::post().to(account_controller::login)),
                    )
                    .service(
                        web::resource("/logout").route(web::post().to(account_controller::logout)),
                    ),
            )
            .service(
                web::scope("/schematic")
                    .service(
                        web::resource("/upload").route(web::post().to(schematic_controller::upload)),
                    )
                    .service(
                        web::resource("/search").route(web::get().to(schematic_controller::search)),
                    )
                    .service(
                        web::resource("/{id}/delete").route(web::post().to(schematic_controller::delete)),
                    )
                    .service(
                        web::resource("/{id}/update").route(web::post().to(schematic_controller::update)),
                    ),
            )
    );
}
