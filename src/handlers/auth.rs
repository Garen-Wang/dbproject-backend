use actix_web::{web, HttpResponse};

use crate::{
    dbaccess::auth::{auth_login_db, auth_register_db, auth_update_db},
    error::MyError,
    models::auth::{LoginForm, RegisterForm, UpdateForm},
    state::AppState,
};

pub async fn auth_login(
    app_state: web::Data<AppState>,
    form: web::Json<LoginForm>,
) -> Result<HttpResponse, MyError> {
    auth_login_db(&app_state.db, form.into())
        .await
        .map(|info| HttpResponse::Ok().json(info))
}

pub async fn auth_register(
    app_state: web::Data<AppState>,
    form: web::Json<RegisterForm>,
) -> Result<HttpResponse, MyError> {
    auth_register_db(&app_state.db, form.into())
        .await
        .map(|info| HttpResponse::Ok().json(info))
}

pub async fn auth_update(app_state: web::Data<AppState>, form: web::Json<UpdateForm>) -> Result<HttpResponse, MyError> {
    auth_update_db(&app_state.db, form.into())
        .await
        .map(|info| HttpResponse::Ok().json(info))
}
