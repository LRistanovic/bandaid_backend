use std::env;

use axum::http::StatusCode;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
    iss: String,
    sub: String,
}

pub fn create_jwt(user_id: &i32) -> Result<String, StatusCode> {
    let now = chrono::Utc::now();
    let exp = now + chrono::Duration::minutes(20);
    let claims = Claims {
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        iss: env::var("ISSUER").unwrap(),
        sub: user_id.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn validate(token: &str) -> Result<i32, StatusCode> {
    let key = DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[env::var("ISSUER").unwrap()]);

    let token_data =
        decode::<Claims>(token, &key, &validation).map_err(|_| StatusCode::BAD_REQUEST)?;

    let user_id = token_data
        .claims
        .sub
        .parse::<i32>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(user_id)
}
