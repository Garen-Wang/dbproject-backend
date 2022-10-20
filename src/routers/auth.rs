use actix_web::web;

use crate::handlers::auth::{auth_login, auth_register, auth_update};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::get().to(auth_login))
            .route("/register", web::post().to(auth_register))
            .route("/update", web::put().to(auth_update)),
        // .service(web::resource("/login").to(auth_login))
        // .service(web::resource("/register").to(auth_register))
        // .service(web::resource("/update").to(auth_update))

        // .route("/login", web::post().to(auth_login))
        // .route("/register", web::post().to(auth_register))
    );
}
