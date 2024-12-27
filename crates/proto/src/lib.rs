use chrono::{DateTime, Utc, NaiveDate};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ContactFormRequest {
    pub name: String,
    pub email: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_slot: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[cfg_attr(feature = "csr", derive(Deserialize))]
pub struct ContactFormResponse {
    pub result: String,
}

pub struct ContactSlot {
    pub date: NaiveDate,
    pub hour: u32,
    pub duration: u32,
}

#[derive(Debug, Clone)]
pub struct ContactForm {

}
