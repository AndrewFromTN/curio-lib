use jsonwebtoken::errors::Result;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

const TOKEN_PREFIX: &str = "Bearer ";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub nameid: String,
    pub unique_name: String,
    pub nbf: usize,
    pub exp: usize,
    pub iat: usize,
}

pub fn decode_and_validate_token(secret: &str, token: &str) -> Result<Claims> {
    decode::<Claims>(
        token.trim_start_matches(TOKEN_PREFIX),
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
}
