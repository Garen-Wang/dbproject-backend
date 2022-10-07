use std::fmt::Display;

use serde::Serialize;

#[derive(Debug)]
pub enum MyError {
    DbError(String),
    ActixError(String),
    NotFound(String),
    UnknownError(String),
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl MyError {
    fn error_msg(&self) -> String {
        match self {
            MyError::DbError(s) => {
                log::error!("db error: {}", s);
                s.into()
            }
            MyError::ActixError(s) => {
                log::error!("actix error: {}", s);
                s.into()
            }
            MyError::NotFound(s) => {
                log::error!("not found: {}", s);
                s.into()
            }
            MyError::UnknownError(s) => {
                log::error!("unknown error: {}", s);
                s.into()
            }
        }
    }
}

#[derive(Serialize)]
struct MyErrorMsg {
    error_msg: String,
}

impl actix_web::error::ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            MyError::DbError(_) | MyError::ActixError(_) => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            MyError::NotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            MyError::UnknownError(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        actix_web::HttpResponse::build(self.status_code()).json(MyErrorMsg {
            error_msg: self.error_msg(),
        })
    }
}

impl From<sqlx::Error> for MyError {
    fn from(e: sqlx::Error) -> Self {
        MyError::DbError(e.to_string())
    }
}
