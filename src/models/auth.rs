use actix_web::web;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

impl From<web::Json<LoginForm>> for LoginForm {
    fn from(form: web::Json<LoginForm>) -> Self {
        Self {
            email: form.email.clone(),
            password: form.password.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<web::Json<RegisterForm>> for RegisterForm {
    fn from(form: web::Json<RegisterForm>) -> Self {
        Self {
            username: form.username.clone(),
            email: form.email.clone(),
            password: form.password.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateForm {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl From<web::Json<UpdateForm>> for UpdateForm {
    fn from(form: web::Json<UpdateForm>) -> Self {
        Self {
            id: form.id,
            username: form.username.clone(),
            password: form.password.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
}
