use serde::{de::Error, Deserialize, Deserializer, Serializer};
use time::{macros::format_description, OffsetDateTime};

pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let formatted = date.format(&format).unwrap();
    serializer.serialize_str(&formatted)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let s = String::deserialize(deserializer)?;
    OffsetDateTime::parse(&s, &format).map_err(D::Error::custom)
}
