use serde::{de::Error, Deserialize, Deserializer, Serializer};
use time::{macros::{format_description, offset}, OffsetDateTime};

pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let formatted = date.to_offset(offset!(+8)).format(&format).unwrap();
    serializer.serialize_str(&formatted)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let mut s = String::deserialize(deserializer)?;
    s += " +08:00:00";
    OffsetDateTime::parse(&s, &format).map_err(D::Error::custom)
}
