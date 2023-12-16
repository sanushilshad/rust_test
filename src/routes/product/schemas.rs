use crate::impl_serialize_format;
use crate::utils::fmt_json;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display};

impl_serialize_format!(InventoryRequest, Display);
#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct ProductInventory {
    #[sqlx(rename = "code")]
    product_code: String,
    #[sqlx(rename = "no_of_items")]
    qty: BigDecimal,
}

impl_serialize_format!(MyResponse, Display);
#[derive(Serialize, Deserialize)]
pub struct MyResponse {
    pub status: bool,
    pub customer_message: String,
    pub success_code: String,
    pub data: Vec<ProductInventory>,
}

impl_serialize_format!(InventoryRequest, Debug);
#[derive(Deserialize, Serialize)]
pub struct InventoryRequest {
    username: String,
    session_id: String,
    product_codes: Vec<String>,
}