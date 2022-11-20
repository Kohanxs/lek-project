use std::{sync::Arc};

use async_trait::async_trait;
use bcrypt;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use rocket::{request::{FromRequest, Outcome}, http::Status};
use crate::models::user::SafeUser;
use crate::utils::BackendError;
use crate::database::{get_safeuser_by_id, DbConn};

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
    sub: String,         // Optional. Subject (whom token refers to)
    token_type: String
}

pub fn generate_access_token(user_id: i32, timestamp: u64, jwt_conf: &JWTConfig) -> Result<String, jsonwebtoken::errors::Error>{
    let access_claim = Claims {
        sub: user_id.to_string(),
        iat: timestamp,
        exp: timestamp + 3600,
        iss: "LEK-backend".to_owned(),
        nbf: timestamp,
        token_type: "access".to_owned(),
    };

    let access = encode(&Header::default(), &access_claim, &jwt_conf.encoding_key)?;

    Ok(access)
}

pub fn generate_refresh_token(user_id: i32, timestamp: u64, jwt_conf: &JWTConfig) -> Result<String, jsonwebtoken::errors::Error> {
    
    let refresh_claim = Claims {
        sub: user_id.to_string(),
        iat: timestamp,
        exp: timestamp + 86400,
        iss: "LEK-backend".to_owned(),
        nbf: timestamp + 3599,
        token_type: "refresh".to_owned(),
    };

    
    let refresh  = encode(&Header::default(), &refresh_claim, &jwt_conf.encoding_key)?;
    Ok(refresh)
}

pub fn validate_token(token: &str, jwt_config: &JWTConfig) -> Result<Claims, BackendError>{
    let validation_result = decode::<Claims>(token, &jwt_config.decoding_key, &jwt_config.validation_setup)?;
    Ok(validation_result.claims)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_validation() {
        use crate::auth::{get_jwt_config, validate_token, generate_access_token};
        let config = get_jwt_config();
        let token = generate_access_token(2, jsonwebtoken::get_current_timestamp(), &config).unwrap();
        let validation_setup = validate_token(&token, &config);
        assert!(validation_setup.is_ok())
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

        let token = request.headers().get_one("Bearer");
        
        match token {
            Some(t) => {
                let config = request.guard::<&rocket::State<Arc<JWTConfig>>>().await;
                let database = request.guard::<DbConn>().await;

                match (config, database) {
                    (Outcome::Success(conf), Outcome::Success(conn)) => {
                        let validation_result = validate_token(t, conf.inner());
                        match validation_result {
                            Ok(claim) => {
                                // TODO get the refresh somehow
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