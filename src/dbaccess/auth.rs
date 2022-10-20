use sqlx::PgPool;

use crate::{models::auth::{LoginForm, RegisterForm, UserInfo, UpdateForm}, error::MyError};

pub async fn auth_login_db(pool: &PgPool, form: LoginForm) -> Result<UserInfo, MyError> {
    let row = sqlx::query!(
        r#"select * from account where a_email = $1"#,
        form.email,
    ).fetch_one(pool).await?;
    if row.a_password.trim_end() == form.password {
        log::info!("account login successfully");
        Ok(UserInfo { id: row.a_id, username: row.a_username, email: row.a_email })
    } else {
        log::info!("account login failed");
        Err(MyError::UnknownError("wrong password".into()))
    }
}

pub async fn auth_register_db(pool: &PgPool, form: RegisterForm) -> Result<UserInfo, MyError> {
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
        let row = sqlx::query!(
            r#"insert into account (a_email, a_username, a_password) values ($1, $2, $3) returning *"#,
            form.email, form.username, form.password,
        ).fetch_one(pool).await?;
        Ok(UserInfo { id: row.a_id, username: row.a_username, email: row.a_email })
    }
}

pub async fn auth_update_db(pool: &PgPool, form: UpdateForm) -> Result<UserInfo, MyError> {
    log::debug!("auth update db");
    let original_row = sqlx::query!(
        r#"select * from account where a_id = $1"#, form.id
    ).fetch_one(pool).await?;
    let row = sqlx::query!(
        r#"update account set a_username = $1, a_password = $2 where a_id = $3 returning *"#,
        if let Some(username) = form.username { username } else { original_row.a_username },
        if let Some(password) = form.password { password } else { original_row.a_password },
        form.id,
    ).fetch_one(pool).await?;
    Ok(UserInfo { id: form.id, username: row.a_username, email: row.a_email })
}
