use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use super::types::ResponsePayload;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = "dshjkvfshfvkjdjdchsjfvcdbnk".to_string();
    Keys::new(secret.as_bytes())
});

pub async fn get_token(claims: Claims) -> anyhow::Result<String> {
    Ok(encode(&Header::default(), &claims, &KEYS.encoding)?)
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    fn from_request_parts<'a, 'b, 'c>(
        parts: &'a mut Parts,
        _state: &'b S,
    ) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<Self, Self::Rejection>> + Send + 'c>>
    where
        'a: 'c,
        'b: 'c,
        Self: 'c,
    {
        Box::pin(async move {
            // Extract the token from the authorization header
            let TypedHeader(Authorization(bearer)) = parts
                .extract::<TypedHeader<Authorization<Bearer>>>()
                .await
                .map_err(|_| AuthError::NotFoundToken)?;
            // Decode the user data
            let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|err| AuthError::InvalidToken(err))?;

            Ok(token_data.claims)
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // token
    pub sub: String,
    pub exp: i64,
    // user
    pub user_id: String,
    pub user_name: String,
    pub user_email: String,
}

impl Claims {
    pub fn new(sub: String, exp: i64, user_id: String, user_name: String, user_email: String) -> Self {
        Self {
            sub,
            exp,
            user_id,
            user_name,
            user_email,
        }
    }
}

pub enum AuthError {
    NotFoundToken,
    InvalidToken(Error),
    #[allow(unused)]
    ExpiredToken,
    #[allow(unused)]
    TokenCreation
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::NotFoundToken => (StatusCode::UNAUTHORIZED, "Token not found".to_string()),
            AuthError::InvalidToken(err) => (StatusCode::UNAUTHORIZED, err.to_string()),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Expired token".to_string()),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating token".to_string()),
        };

        let message = Json(ResponsePayload::<()> {
            status: status.as_u16(),
            message: message.to_string(),
            data: None,
        });

        (status, message).into_response()
    }
}

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
