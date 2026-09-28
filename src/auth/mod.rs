use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, TokenData};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::auth::TokenType::Access;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

pub struct AuthService {
    jwt_secret: String,
    access_expiry: i64,
    refresh_expiry: i64,
}

impl AuthService {

    pub fn new(jwt_secret:String, access_expiry:i64, refresh_expiry: i64) -> Self {
        Self {
            jwt_secret,
            access_expiry,
            refresh_expiry,
        }
    }

    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError>{
        hash(password, DEFAULT_COST)
    }

    pub fn verify_password(password: &str,  hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash)
    }

    pub fn generate_access_token(&self, user_id: &str, user_email: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let timenow = Utc::now();
        let timeexp = timenow + Duration::minutes(self.access_expiry);
        let collectinClaims = Claims {
            sub: user_id.to_string(),
            email: user_email.to_string(),
            exp: timeexp.timestamp() as usize,
            iat: timenow.timestamp() as usize,
            token_type: TokenType::Access,
        };

        encode(
            &Header::default(), 
            &collectinClaims, 
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
    }

    pub fn generate_refresh_token(&self, user_id: &str, user_email: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let timenow_generate_refresh_token = Utc::now();
        let timeexp_generate_refresh_token = timenow_generate_refresh_token + Duration::days(self.refresh_expiry);

        let collectingClaims= Claims {
            sub: user_id.to_string(),
            email: user_email.to_string(),
            exp: timeexp_generate_refresh_token.timestamp() as usize,
            iat: timenow_generate_refresh_token.timestamp() as usize,
            token_type: TokenType::Refresh,
        };

        encode(
            &Header::default(), 
            &collectingClaims, 
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
    )
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token, 
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
             &Validation::default(),
            )
    }

    pub fn generate_verification_token(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub fn generate_reset_token(&self) -> String {
        Uuid::new_v4().to_string()
    }

}

    //Middleware 
    use actix_web::{dev::ServiceRequest, Error, HttpMessage};
    use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
    use actix_web_httpauth::extractors::AuthenticationError;


pub async fn jwt_validator(serviceRequest: ServiceRequest, credentials: BearerAuth,) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    
    let config = serviceRequest.app_data::<Config>().cloned().unwrap_or_default();
    let authservice = serviceRequest.app_data::<AuthService>().unwrap();

    match authservice.verify_token(credentials.token()) {
        Ok(token_data) => {
            serviceRequest.extensions_mut().insert(token_data);
            Ok(serviceRequest)
        }

        Err(_) => Err((AuthenticationError::from(config).into(), serviceRequest)),
    }
}