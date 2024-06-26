use crate::errors::GenericError;
use crate::utils::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum AuthError {
    #[error("{0}")]
    InvalidCredentials(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    UnexpectedCustomError(String),
    #[error("{0}")]
    DatabaseError(String, anyhow::Error),
    // #[error("{0}")]
    // InvalidJWT(String),
}

impl std::fmt::Debug for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<AuthError> for GenericError {
    fn from(err: AuthError) -> GenericError {
        match err {
            AuthError::UnexpectedError(error) => GenericError::UnexpectedError(error),
            AuthError::DatabaseError(message, error) => GenericError::DatabaseError(message, error),
            AuthError::ValidationError(message) => GenericError::ValidationError(message),
            AuthError::InvalidCredentials(message) => GenericError::ValidationError(message),
            AuthError::UnexpectedCustomError(message) => GenericError::ValidationError(message),
        }
    }
}

// impl ResponseError for AuthError {
//     fn status_code(&self) -> StatusCode {
//         match self {
//             AuthError::InvalidCredentials(_) => StatusCode::BAD_REQUEST,
//             AuthError::InvalidStringCredentials(_) => StatusCode::BAD_REQUEST,
//             AuthError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
//             // AuthError::ValidationError(_) => StatusCode::BAD_REQUEST,
//             AuthError::ValidationStringError(_) => StatusCode::BAD_REQUEST,
//             AuthError::UnexpectedStringError(_) => StatusCode::INTERNAL_SERVER_ERROR,
//             AuthError::DatabaseError(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
//             AuthError::InvalidJWT(_) => StatusCode::UNAUTHORIZED,
//         }
//     }

//     fn error_response(&self) -> HttpResponse {
//         let status_code = self.status_code();
//         let status_code_str = status_code.as_str();
//         let inner_error_msg = match self {
//             AuthError::InvalidCredentials(inner_error)
//             | AuthError::UnexpectedError(inner_error) => inner_error.to_string(),
//             // | AuthError::ValidationError(inner_error) => inner_error.to_string(),
//             AuthError::ValidationStringError(message) => message.to_string(),
//             AuthError::UnexpectedStringError(message) => message.to_string(),
//             AuthError::DatabaseError(message, _err) => message.to_string(),
//             AuthError::InvalidStringCredentials(message) => message.to_string(),
//             AuthError::InvalidJWT(message) => message.to_string(),
//         };

//         HttpResponse::build(status_code).json(GenericResponse::error(
//             &inner_error_msg,
//             status_code_str,
//             Some(()),
//         ))
//     }
// }

#[derive(thiserror::Error)]
pub enum UserRegistrationError {
    #[error("Duplicate email")]
    DuplicateEmail(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Duplicate mobile no")]
    DuplicateMobileNo(String),
    #[error("{0}")]
    DatabaseError(String, anyhow::Error),
    #[error("Insufficient previlege to register Admin/Superadmin")]
    InsufficientPrevilegeError(String),
    #[error("Invalid Role")]
    InvalidRoleError(String),
}

impl std::fmt::Debug for UserRegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<UserRegistrationError> for GenericError {
    fn from(err: UserRegistrationError) -> GenericError {
        match err {
            UserRegistrationError::DuplicateEmail(message) => {
                GenericError::ValidationError(message)
            }
            UserRegistrationError::DuplicateMobileNo(message) => {
                GenericError::ValidationError(message)
            }
            UserRegistrationError::UnexpectedError(error) => GenericError::UnexpectedError(error),
            UserRegistrationError::DatabaseError(message, error) => {
                GenericError::DatabaseError(message, error)
            }
            UserRegistrationError::InvalidRoleError(message) => {
                GenericError::ValidationError(message)
            }
            UserRegistrationError::InsufficientPrevilegeError(message) => {
                GenericError::InsufficientPrevilegeError(message)
            }
        }
    }
}

// impl ResponseError for UserRegistrationError {
//     fn status_code(&self) -> StatusCode {
//         match self {
//             UserRegistrationError::DuplicateEmail(_) => StatusCode::BAD_REQUEST,
//             UserRegistrationError::DuplicateMobileNo(_) => StatusCode::BAD_REQUEST,
//             UserRegistrationError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
//             UserRegistrationError::DatabaseError(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
//             UserRegistrationError::InsufficientPrevilegeError(_) => StatusCode::UNAUTHORIZED,
//             UserRegistrationError::InvalidRoleError(_) => StatusCode::UNAUTHORIZED,
//         }
//     }

//     fn error_response(&self) -> HttpResponse {
//         let status_code = self.status_code();
//         let status_code_str = status_code.as_str();
//         let inner_error_msg = match self {
//             UserRegistrationError::DuplicateEmail(inner_error)
//             | UserRegistrationError::DuplicateMobileNo(inner_error)
//             | UserRegistrationError::UnexpectedError(inner_error) => inner_error.to_string(),
//             UserRegistrationError::DatabaseError(error_msg, _err) => error_msg.clone(),
//             UserRegistrationError::InsufficientPrevilegeError(error_msg) => error_msg.to_string(),
//             UserRegistrationError::InvalidRoleError(error_msg) => error_msg.to_string(),
//         };

//         HttpResponse::build(status_code).json(GenericResponse::error(
//             &inner_error_msg,
//             status_code_str,
//             Some(()),
//         ))
//     }
// }
#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error)]

pub enum BusinessAccountError {
    // #[error("Insufficient previlege to register Admin/Superadmin")]
    // InsufficientPrevilegeError(String),
    #[error("{0}, {1}")]
    DatabaseError(String, anyhow::Error),
    #[error("Invalid Role")]
    InvalidRoleError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for BusinessAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl From<BusinessAccountError> for GenericError {
    fn from(err: BusinessAccountError) -> GenericError {
        match err {
            BusinessAccountError::UnexpectedError(error) => GenericError::UnexpectedError(error),
            BusinessAccountError::DatabaseError(message, error) => {
                GenericError::DatabaseError(message, error)
            }
            BusinessAccountError::InvalidRoleError(message) => {
                GenericError::ValidationError(message)
            }
        }
    }
}

// impl ResponseError for BusinessAccountError {
//     fn status_code(&self) -> StatusCode {
//         match self {
//             // BusinessRegistrationError::InsufficientPrevilegeError(_) => StatusCode::UNAUTHORIZED,
//             BusinessAccountError::DatabaseError(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
//             BusinessAccountError::InvalidRoleError(_) => StatusCode::UNAUTHORIZED,
//             BusinessAccountError::UnexpectedError(_) => StatusCode::UNAUTHORIZED,
//             BusinessAccountError::ValidationStringError(_) => StatusCode::BAD_REQUEST,
//             BusinessAccountError::UnexpectedStringError(_) => StatusCode::BAD_REQUEST,
//         }
//     }

//     fn error_response(&self) -> HttpResponse {
//         let status_code = self.status_code();
//         let status_code_str = status_code.as_str();
//         let inner_error_msg = match self {
//             BusinessAccountError::DatabaseError(message, _err) => message.to_string(),
//             BusinessAccountError::InvalidRoleError(error_msg) => error_msg.to_string(),
//             BusinessAccountError::ValidationStringError(message) => message.to_string(),
//             // BusinessRegistrationError::InsufficientPrevilegeError(error_msg) => {
//             //     error_msg.to_string()
//             // }
//             BusinessAccountError::UnexpectedError(error_msg) => error_msg.to_string(),
//             BusinessAccountError::UnexpectedStringError(error_msg) => error_msg.to_string(),
//         };

//         HttpResponse::build(status_code).json(GenericResponse::error(
//             &inner_error_msg,
//             status_code_str,
//             Some(()),
//         ))
//     }
// }
