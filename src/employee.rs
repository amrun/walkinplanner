use chrono::NaiveDate;
use serde::de::{self, Deserializer};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
    pub surname: String,
    pub short: String,
    pub percent: f32,
    pub off_days: String,
    pub id: u32,
    pub count: f32,
    #[serde(
        serialize_with = "serialize_naivedate",
        deserialize_with = "deserialize_naivedate"
    )]
    pub last_duty: NaiveDate,
}

// Custom serialization function for NaiveDate to string
fn serialize_naivedate<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format("%d.%m.%Y").to_string(); // Format as "YYYY-MM-DD"
    serializer.serialize_str(&s)
}

// Custom deserialization function for NaiveDate from string
fn deserialize_naivedate<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?; // Get the string from JSON
    NaiveDate::parse_from_str(&s, "%d.%m.%Y").map_err(de::Error::custom) // Parse it
}
