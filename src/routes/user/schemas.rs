use secrecy::Secret;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgHasArrayType;
use std::fmt::Debug;
use uuid::Uuid;

use crate::domain::{subscriber_email::deserialize_subscriber_email, EmailObject};

// macro_rules! impl_serialize_format {
//     ($struct_name:ident, $trait_name:path) => {
//         impl $trait_name for $struct_name {
//             fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//                 fmt_json(self, f)
//             }
//         }
//     };
// }

// #[derive(Debug)]
// struct SecretString(Secret<String>);

// impl_serialize_format!(AuthenticateRequest, Debug);
// #[strum(serialize_all = "snake_case")]
// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "lowercase")]

// pub enum AuthenticationScope {
//     OTP,
//     Password,
// }
// impl_serialize_format!(AuthenticateRequest, Display);
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateRequest {
    pub scope: AuthenticationScope,
    pub identifier: String,
    // #[serde(with = "SecretString")]
    pub secret: Secret<String>,
}

// impl Serialize for SecretString {
//     fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//         serializer.serialize_str(&self.0.expose_secret())
//     }
// }

// pub struct AuthData {
//     token: String,
// }

// pub struct AuthResponse {
//     data: AuthData,
// }

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "user_auth_identifier_scope", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AuthenticationScope {
    Otp,
    Password,
    Google,
    Facebook,
    Microsoft,
    Apple,
    Token,
    AuthApp,
    Qr,
    Email,
}

impl PgHasArrayType for AuthenticationScope {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_user_auth_identifier_scope")
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserAccount {
    pub username: String,
    pub mobile_no: String,
    pub international_dialing_code: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_subscriber_email")]
    pub email: EmailObject, //NOTE: email_address crate cah be used if needed,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MaskingType {
    NA,
    Encrypt,
    PartialMask,
    FullMask,
}
#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "header_pair")]
pub struct UserVectors {
    pub key: String,
    pub value: String,
    pub masking: MaskingType,
}

// impl PgHasArrayType for UserVectors {
//     fn array_type_info() -> sqlx::postgres::PgTypeInfo {
//         sqlx::postgres::PgTypeInfo::with_name("_header_pair")
//     }
// }

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct JWTClaims {
    pub sub: Uuid,
    pub exp: usize,
}
