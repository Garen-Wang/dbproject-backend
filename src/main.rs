use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use error::MyError;
use routers::auth::auth_routes;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use std::sync::Mutex;

pub mod error;
pub mod state;
pub mod routers;
pub mod handlers;
pub mod models;
pub mod dbaccess;

#[actix_rt::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pg_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        vis_count: Mutex::new(0),
        db: pg_pool,
        // courses: Mutex::new(vec![])
    });
    let factory = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000/")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::UnknownError("invalid json".into()).into()
            }))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(auth_routes)
    };
    HttpServer::new(factory).bind("localhost:7878")?.run().await
}
