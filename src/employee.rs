use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
    pub surname: String,
    pub short: String,
    pub percent: f32,
    #[serde(rename = "fixDays")]
    pub fix_days: String,
    #[serde(rename = "offDays")]
    pub off_days: String,
    pub id: u32,
    pub count: u32,
}
