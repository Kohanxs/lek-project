use std::{sync::Arc};

use async_trait::async_trait;
use bcrypt;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use rocket::{request::{FromRequest, Outcome}, http::Status};
use crate::models::user::SafeUser;
use crate::utils::BackendError;
use crate::database::{get_safeuser_by_id, DbConn};

pub enum TokenType{
    Access,
    Refresh
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            TokenType::Access => "access",
            TokenType::Refresh => "refresh"
        }
    }
}

pub fn get_hash(password: &str) -> Result<String, bcrypt::BcryptError>{
    let hashing_result = bcrypt::hash_with_result(password, bcrypt::DEFAULT_COST)?;
    Ok(hashing_result.format_for_version(bcrypt::Version::TwoB))
}

pub fn check_hash(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError>{
    bcrypt::verify(password, hash)
}

pub struct JWTConfig {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation_setup: Validation
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: u64,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: u64,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: u64,          // Optional. Not Before (as UTC timestamp)
    pub sub: String,         // Optional. Subject (whom token refers to)
    token_type: String,         // Optional. Token type - access or refresh
    pub admin: bool
}

pub fn generate_access_token(user_id: i32, timestamp: u64, jwt_conf: &JWTConfig, is_admin: bool) -> Result<String, jsonwebtoken::errors::Error>{
    let access_claim = Claims {
        sub: user_id.to_string(),
        iat: timestamp,
        exp: timestamp + 3600,
        iss: "LEK-backend".to_owned(),
        nbf: timestamp,
        token_type: TokenType::Access.as_str().to_owned(),
        admin: is_admin
    };

    let access = encode(&Header::default(), &access_claim, &jwt_conf.encoding_key)?;

    Ok(access)
}

pub fn generate_refresh_token(user_id: i32, timestamp: u64, jwt_conf: &JWTConfig, is_admin: bool) -> Result<String, jsonwebtoken::errors::Error> {
    
    let refresh_claim = Claims {
        sub: user_id.to_string(),
        iat: timestamp,
        exp: timestamp + 86400,
        iss: "LEK-backend".to_owned(),
        nbf: timestamp + 3599,
        token_type: TokenType::Refresh.as_str().to_owned(),
        admin: is_admin
    };

    
    let refresh  = encode(&Header::default(), &refresh_claim, &jwt_conf.encoding_key)?;
    Ok(refresh)
}


pub fn validate_token(token: &str, jwt_config: &JWTConfig, token_type: TokenType) -> Result<Claims, BackendError>{
    let validation_result = decode::<Claims>(token, &jwt_config.decoding_key, &jwt_config.validation_setup)?;
    if validation_result.claims.token_type.eq(token_type.as_str()) {
        Ok(validation_result.claims)
    } else {
        Err(BackendError::WrongCredentials)
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::TokenType;

    #[test]
    fn test_validation_access() {
        use crate::auth::{get_jwt_config, validate_token, generate_access_token};
        let config = get_jwt_config();
        let token = generate_access_token(2, jsonwebtoken::get_current_timestamp(), &config, false).unwrap();
        let validation_setup = validate_token(&token, &config, TokenType::Access);
        assert!(validation_setup.is_ok());
        assert!(validation_setup.unwrap().token_type.eq("access") )
    }

    #[test]
    fn test_validation_refresh() {
        use crate::auth::{get_jwt_config, validate_token, generate_refresh_token};
        let mut config = get_jwt_config();
        config.validation_setup.validate_nbf = false;
        let token = generate_refresh_token(2, jsonwebtoken::get_current_timestamp(), &config, false).unwrap();
        let validation_setup = validate_token(&token, &config, TokenType::Refresh);
        assert!(validation_setup.is_ok());
    }

    #[test]
    fn test_validation_refresh_not_access() {
        use crate::auth::{get_jwt_config, validate_token, generate_refresh_token};
        let mut config = get_jwt_config();
        config.validation_setup.validate_nbf = false;
        let token = generate_refresh_token(2, jsonwebtoken::get_current_timestamp(), &config, false).unwrap();
        let validation_setup = validate_token(&token, &config, TokenType::Access);
        assert!(validation_setup.is_err());
    }
}

pub fn get_jwt_config() -> JWTConfig{
    let secret = "secret";
    let mut validation_setup = Validation::new(Algorithm::HS256);
    validation_setup.set_issuer(&["LEK-backend"]);
    validation_setup.validate_nbf = true;
    validation_setup.set_required_spec_claims(&["exp", "nbf", "iss", "sub"]);
    JWTConfig {
        encoding_key: EncodingKey::from_secret(secret.as_ref()),
        decoding_key: DecodingKey::from_secret(secret.as_ref()),
        validation_setup: validation_setup
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for SafeUser {
    type Error = BackendError;
    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {

        let bearer_token = request.headers().get_one("Authorization");
        
        match bearer_token {
            Some(token_str) => {
                let parsed_token = match token_str.strip_prefix("Bearer ") {
                    None => return Outcome::Failure((Status::BadRequest, BackendError::NotAuthorized)),
                    Some(parsed) => parsed
                };
                let config = request.guard::<&rocket::State<Arc<JWTConfig>>>().await;
                let database = request.guard::<DbConn>().await;

                match (config, database) {
                    (Outcome::Success(conf), Outcome::Success(conn)) => {
                        let validation_result = validate_token(parsed_token, conf.inner(), TokenType::Access);
                        match validation_result {
                            Ok(claim) => {
                                let user_id = claim.sub.parse::<i32>();
                                match user_id {
                                    Ok(user_id) => {
                                        let user_result = conn.run(move |conn| get_safeuser_by_id(conn, user_id)).await;
                                        match user_result {
                                            Ok(user) => return Outcome::Success(user),
                                            Err(e) => return Outcome::Failure((Status::Unauthorized, e))
                                        };
                                    },
                                    Err(_) => return Outcome::Failure((Status::BadRequest, BackendError::NotAuthorized))
                                }
                                
                            },
                            Err(e) => return Outcome::Failure((Status::InternalServerError, e))
                        };
                    },
                    _ => Outcome::Failure((Status::InternalServerError,BackendError::UnknownError))
                }
                
            },
            None => return Outcome::Forward(()) //No token
        }
    }
}