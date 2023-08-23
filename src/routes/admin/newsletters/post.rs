use actix_web::{
    http::header::{self},
    web, HttpResponse, ResponseError,
};
use actix_web_flash_messages::FlashMessage;
use anyhow::Context;
use reqwest::{header::HeaderValue, StatusCode};
use sqlx::PgPool;

use crate::{
    authentication::UserId,
    domain::SubscriberEmail,
    email_client::EmailClient,
    routes::{admin::dashboard::get_username, error_chain_fmt},
    utils::see_other,
};

/* #[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html_content: String,
    text_content: String,
} */

#[derive(serde::Deserialize)]
pub struct FormNewsletter {
    title: String,
    text_content: String,
    html_content: String,
}

#[tracing::instrument(
    name = "Publishing new newsletter",
    skip(body, pool, email_client, user_id),
    fields(user_id=%*user_id)
)]
pub async fn publish_newsletter(
    body: web::Form<FormNewsletter>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    user_id: web::ReqData<UserId>,
) -> Result<HttpResponse, PublishError> {
    let subscribers = get_confirmed_subscribers(&pool).await?;
    for subscriber in subscribers {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_email(
                        &subscriber.email,
                        &body.title,
                        &body.html_content,
                        &body.text_content,
                    )
                    .await
                    .with_context(|| {
                        format!("Failed to send newsletter to {}", subscriber.email)
                    })?;
            }
            Err(error) => {
                tracing::warn!(
                    error.cause_chain = ?error,
                    "Skipping confirmed subscriber\
                    Their stored email now are invalid."
                );
            }
        }
    }
    FlashMessage::info("Successfully send a newsletter").send();
    Ok(see_other("/admin/newsletters"))
}

struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let confirmed_subscribers =
        sqlx::query!("SELECT email FROM subscriptions WHERE status = 'confirmed'")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|r| match SubscriberEmail::parse(r.email) {
                Ok(email) => Ok(ConfirmedSubscriber { email }),
                Err(e) => Err(anyhow::anyhow!(e)),
            })
            .collect();

    Ok(confirmed_subscribers)
}

#[derive(thiserror::Error)]
pub enum PublishError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("Authentication failed.")]
    AuthError(#[source] anyhow::Error),
}

impl std::fmt::Debug for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PublishError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::UnexpectedError(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
            Self::AuthError(_) => {
                let mut response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                let header_value = HeaderValue::from_str(r#"Basic realm="publish""#).unwrap();
                response
                    .headers_mut()
                    .insert(header::WWW_AUTHENTICATE, header_value);
                response
            }
        }
    }
}

// pub fn basic_authentication(headers: &HeaderMap) -> Result<Credentials, anyhow::Error> {
//     let header_value = headers
//         .get("Authorization")
//         .context("Authorization header is missing")?
//         .to_str()
//         .context("The 'Authorization' header was not a valid UTF8 string.")?;
//     let base64encoded_segment = header_value
//         .strip_prefix("Basic ")
//         .context("The authorization scheme was not 'Basic'.")?;
//     let decoded_bytes = general_purpose::STANDARD
//         .decode(base64encoded_segment)
//         .context("Failed to base64-decode 'Basic' credentials.")?;
//     let decoded_credentials = String::from_utf8(decoded_bytes)
//         .context("The decoded credential string is not valid UTF8.")?;
//
//     let mut credentials = decoded_credentials.splitn(2, ':');
//     let username = credentials
//         .next()
//         .ok_or_else(|| anyhow::anyhow!("A username must be provided in 'Basic' auth."))?
//         .to_string();
//     let password = credentials
//         .next()
//         .ok_or_else(|| anyhow::anyhow!("A password must be provided in 'Basic' auth."))?
//         .to_string();
//     Ok(Credentials {
//         username,
//         password: Secret::new(password),
//     })
// }
