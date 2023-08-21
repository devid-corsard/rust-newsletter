use actix_web::{error::InternalError, web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    authentication::{
        change_password as change_password_in_db, validate_credentials, AuthError, Credentials,
    },
    routes::admin::dashboard::get_username,
    session_state::TypedSession,
    utils::{e500, see_other},
};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Form<FormData>,
    session: TypedSession,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = reject_anonymous_users(session).await?;
    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error("New passwords doesn't match.").send();
        return Ok(see_other("/admin/password"));
    }
    if form.new_password.expose_secret().len() < 8 {
        FlashMessage::error("Password must be at least 8 characters long.").send();
        return Ok(see_other("/admin/password"));
    }
    let username = get_username(&user_id, &pool).await.map_err(e500)?;
    let credentials = Credentials {
        username,
        password: form.0.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("Current password is invalid.").send();
                return Ok(see_other("/admin/password"));
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    };
    change_password_in_db(user_id, form.0.new_password, &pool)
        .await
        .map_err(e500)?;
    FlashMessage::info("Password successfully changed.").send();

    Ok(see_other("/admin/password"))
}

pub async fn reject_anonymous_users(session: TypedSession) -> Result<Uuid, actix_web::Error> {
    match session.get_user_id().map_err(e500)? {
        Some(user_id) => Ok(user_id),
        None => {
            let response = see_other("/login");
            let e = anyhow::anyhow!("The user has not logged in");
            Err(InternalError::from_response(e, response).into())
        }
    }
}
