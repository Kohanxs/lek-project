use std::fmt::Display;

use bcrypt::BcryptError;
use diesel::result::Error;

#[derive(Debug)]
pub enum BackendError {
    CryptoError(BcryptError),
    DatabaseError(Error)
}

impl Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendError::CryptoError(bcrypt_error) => 
                write!(f, "{}", bcrypt_error),
            BackendError::DatabaseError(diesel_error) => 
                write!(f, "{}", diesel_error),
        }
    }
}

impl std::error::Error for BackendError {}

impl From<BcryptError> for BackendError {
    fn from(err: BcryptError) -> Self{
        BackendError::CryptoError(err)
    }
}

impl From<Error> for BackendError {
    fn from(err: Error) -> Self {
        BackendError::DatabaseError(err)
    }
}
