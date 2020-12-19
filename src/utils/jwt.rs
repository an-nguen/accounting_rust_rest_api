use std::env;

use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,         // Optional. Audience
    pub exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize,          // Optional. Issued at (as UTC timestamp)
    pub iss: String,         // Optional. Issuer
    pub nbf: usize,          // Optional. Not Before (as UTC timestamp)
    pub sub: String,         // Optional. Subject (whom token refers to)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonWebKey {
    pub kty: String,
    pub d: String,
    pub crv: String,
    pub kid: String,
    pub x: String,
    pub y: String,
    pub alg: String,
}

pub trait GenerateJwtToken {
    fn generate_token(&self) -> Result<String, jsonwebtoken::errors::Error>;
}

impl GenerateJwtToken for User {
    fn generate_token(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let exp = (Utc::now() + Duration::days(14)).timestamp();
        let iat = Utc::now().timestamp();
        let iss = String::from("actix-web server");
        let claims = Claims {
            aud: self.email.clone().expect("failed to get email string"),
            exp: exp as usize,
            iat: iat as usize,
            iss,
            nbf: 0,
            sub: self.username.clone()
        };

        let secret = read_secret();
        let header = Header::new(Algorithm::HS256);
        let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());
        encode(&header, &claims, &encoding_key)
    }
}

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    decode::<Claims>(&token, &DecodingKey::from_secret(read_secret().as_bytes()), &Validation::default())
}

pub fn verify_token(token_data: TokenData<Claims>, state: &AppState) -> Result<String, String> {
    let conn = state.pool.get().expect("failed to get connection");
    if User::is_valid_token_claim(&token_data.claims, &conn) {
        let now = chrono::Utc::now();
        let exp_naive = NaiveDateTime::from_timestamp(token_data.claims.exp as i64, 0);
        let exp: DateTime<Utc> = DateTime::from_utc(exp_naive, Utc);
        if exp > now {
            Ok(token_data.claims.sub.clone())
        } else {
            Err(String::from("token is expired"))
        }
    } else {
        Err(String::from("invalid token"))
    }
}

pub fn read_secret() -> String {
    env::var("SECRET").unwrap_or_else(|_| String::from("random_secret"))
}