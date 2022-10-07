use actix_web::web;

use crate::handlers::auth::{auth_login, auth_register};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
        .service(web::resource("/login").to(auth_login))
        .service(web::resource("/register").to(auth_register))
        // .service(web::resource("/info").to(auth_info))

        // .route("/login", web::post().to(auth_login))
        // .route("/register", web::post().to(auth_register))
    );
}