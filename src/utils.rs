use std::fmt::Display;
// use thiserror::Error;
use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use jsonwebtoken::errors::Error as JsonWebTokenError;

// pub enum LoginFailureError {
//     // UsernameError(FieldError::new::<DefaultScalarValue>("Password verification failed!", graphql_value!({"verification_error": "Wrong password"}))),
//     UsernameError(),
//     PasswordError()
// }


#[derive(Debug)]
pub enum BackendError {
    CryptoError(BcryptError),
    DatabaseError(DieselError),
    TokenError(JsonWebTokenError),
    UnknownError,
    NotAuthorized,
}

impl Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendError::CryptoError(crypto_error) =>
                write!(f, "{}", crypto_error),
            BackendError::DatabaseError(diesel_error) => 
                write!(f, "{}", diesel_error),
            BackendError::TokenError(err) =>
                write!(f, "{}", err),
            BackendError::UnknownError =>
                write!(f, "Unknown error"),
            BackendError::NotAuthorized =>
                write!(f, "You are not authorized!")
        }
    }
}


impl std::error::Error for BackendError {}

impl From<BcryptError> for BackendError {
    fn from(err: BcryptError) -> Self{
        BackendError::CryptoError(err)
    }
}

impl From<DieselError> for BackendError {
    fn from(err: DieselError) -> Self {
        BackendError::DatabaseError(err)
    }
}

impl From<JsonWebTokenError> for BackendError {
    fn from(err: JsonWebTokenError) -> Self {
        BackendError::TokenError(err)
    }
}