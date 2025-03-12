use serde::{Deserialize, Serialize};
use crate::employee::Employee;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyData {
    pub from: String,
    pub to: String,
    #[serde(rename = "globalOffDays")]
    pub global_off_days: Vec<String>,
    #[serde(rename = "globalHolidays")]
    pub global_holidays: Vec<String>,
    pub employees: Vec<Employee>,
}