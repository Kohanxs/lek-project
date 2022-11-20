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
    WrongCredentials
}

impl Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendError::CryptoError(crypto_error) =>
                write!(f, "{}", crypto_error),
            BackendError::DatabaseError(diesel_error) => {
                match diesel_error {
                    DieselError::NotFound => write!(f, "Resource not found"),
                    DieselError::DatabaseError(error_kind, details) => {
                        match error_kind {
                            diesel::result::DatabaseErrorKind::UniqueViolation => {
                                match details.constraint_name() {
                                    Some("users_username_uindex") => write!(f, "Username already taken"),
                                    Some("users_nickname_uindex") => write!(f, "Nickname already taken"),
                                    _ => write!(f, "Unknown uniqueViolation happened! Contact the administrator")
                                }
                            },
                            diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                                match details.constraint_name() {
                                    Some("comments_questions_id_fk") => write!(f, "No such question found"),
                                    Some("comments_users_id_fk") => write!(f, "No such user found"),
                                    _ => write!(f, "Unknown foreignKeyViolation happened! Contact the administrator")
                                }
                            },
                            _ => write!(f, "{:?}, message {:?}, details {:?}, column_name {:?}", error_kind, details.message(), details.details(), details.constraint_name())
                        }
                    },
                    _ => write!(f, "{}", diesel_error)
                }
            },
            BackendError::TokenError(err) =>
                write!(f, "{}", err),
            BackendError::UnknownError =>
                write!(f, "Unknown error"),
            BackendError::NotAuthorized =>
                write!(f, "You are not authorized!"),
            BackendError::WrongCredentials => 
                write!(f, "Wrong credentials given!")
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