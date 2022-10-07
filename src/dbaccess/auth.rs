use sqlx::PgPool;

use crate::{models::auth::{LoginForm, RegisterForm}, error::MyError};

pub async fn auth_login_db(pool: &PgPool, form: LoginForm) -> Result<LoginForm, MyError> {
    let row = sqlx::query!(
        r#"select * from account where a_email = $1"#,
        form.email,
    ).fetch_one(pool).await?;
    if row.a_password.trim_end() == form.password {
        log::info!("account login successfully");
        Ok(form)
    } else {
        log::info!("account login failed");
        Err(MyError::UnknownError("wrong password".into()))
    }
}

pub async fn auth_register_db(pool: &PgPool, form: RegisterForm) -> Result<RegisterForm, MyError> {
    log::debug!("auth register db");
    let row = sqlx::query!(
        r#"select * from account where a_email = $1"#,
        form.email,
    ).fetch_one(pool).await;

    if row.is_ok() {
        log::info!("account register failed");
        Err(MyError::UnknownError("the account has been registered".into()))
    } else {
        log::info!("account register successfully");
        sqlx::query!(
            r#"insert into account (a_email, a_password) values ($1, $2) returning *"#,
            form.email, form.password,
        ).fetch_one(pool).await?;
        Ok(form)
    }
}