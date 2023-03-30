use actix_web::{
    dev::Payload, error::ErrorUnauthorized, Error as ActixWebError, FromRequest, HttpRequest,
};
use derive_more::Deref;
use jsonwebtoken::{
    decode, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub is_admin: bool,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub user_id: i32,
    pub is_admin: bool,
}

#[derive(Deref, Debug, Serialize, Deserialize)]
pub struct MaybeTokenPayload(pub Option<TokenPayload>);

impl MaybeTokenPayload {
    fn new(user_id: i32, is_admin: bool) -> Self {
        MaybeTokenPayload(Some(TokenPayload{user_id, is_admin}))
    }
}

impl FromRequest for TokenPayload {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        let Some(authorization_header) = req.headers().get("AUTH_TOKEN") else {
            return ready(Err(ErrorUnauthorized(
                "No authentication token!",
            )));
        };

        let Ok(authentication_token) = authorization_header.to_str() else {
            return ready(Err(ErrorUnauthorized(
                "Authentication token has foreign chars!",
            )));
        };

        let secret: &str = &std::env::var("JWT_SECRET").expect("Missing ${JWT_SECRET}");

        let token_result: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
            authentication_token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        );

        ready(
            token_result
                .map(|token| TokenPayload { user_id: token.claims.user_id, is_admin: token.claims.is_admin })
                .map_err(|_| ErrorUnauthorized("Invalid authentication token")),
        )
    }
}
