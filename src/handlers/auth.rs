use actix_web::{web, HttpResponse};

use crate::{
    dbaccess::auth::{auth_login_db, auth_register_db},
    error::MyError,
    models::auth::{LoginForm, RegisterForm},
    state::AppState,
};

pub async fn auth_login(
    app_state: web::Data<AppState>,
    form: web::Json<LoginForm>,
) -> Result<HttpResponse, MyError> {
    auth_login_db(&app_state.db, form.into())
        .await
        .map(|form| HttpResponse::Ok().json(form))
}

pub async fn auth_register(
    app_state: web::Data<AppState>,
    form: web::Json<RegisterForm>,
) -> Result<HttpResponse, MyError> {
    auth_register_db(&app_state.db, form.into())
        .await
        .map(|form| HttpResponse::Ok().json(form))
}
