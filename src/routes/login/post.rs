use actix_web::{
    http::{header::LOCATION, StatusCode},
    web, HttpResponse, ResponseError,
};
use hmac::{Hmac, Mac};
use secrecy::Secret;
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentials, Credentials},
    routes::error_chain_fmt,
};

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        StatusCode::SEE_OTHER
    }

    fn error_response(&self) -> HttpResponse {
        let query_string = format!("error={}", urlencoding::Encoded::new(self.to_string()));

        let secret: &[u8] = todo!();
        let hmac_tag = {
            let mut mac = Hmac::<sha2::Sha256>::new_from_slice(secret).unwrap();
            mac.update(query_string.as_bytes());
            mac.finalize().into_bytes()
        };
        HttpResponse::build(self.status_code())
            // Append the hexadecimal representation of the HMAC tag to the
            // query string as an additional query parameter
            .insert_header((
                LOCATION,
                format!("/login?{}&tag={:x}", query_string, hmac_tag),
            ))
            .finish()
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(
    skip(form, pool),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
)]
pub async fn login(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, LoginError> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };

    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish())
        }
        Err(_) => todo!(),
    }
}
