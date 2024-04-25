use chrono::{DateTime, FixedOffset, Local, NaiveDateTime};
use serde::{de::Error, Deserialize, Deserializer, Serializer};

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&formatted)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(D::Error::custom)?;
    Ok(DateTime::<Local>::from_naive_utc_and_offset(
        dt,
        FixedOffset::east_opt(8 * 3600).unwrap(),
    ))
}
