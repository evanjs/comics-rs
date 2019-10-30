use std::result::Result;
use chrono::{NaiveDateTime, Utc, DateTime};
use serde::de;
use std::option::Option;
use std::result::Result::{Ok, Err};
use std::option::Option::{None, Some};
use std::fmt;
use std::string::String;
use serde::{Deserialize, Deserializer};

pub fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

/// Formatter for Optional DateTime values
/// [Source](https://chrismcg.com/2019/04/30/deserializing-optional-datetimes-with-serde/)
struct OptionalDateTimeFromCustomFormatVisitor;

impl<'de> de::Visitor<'de> for OptionalDateTimeFromCustomFormatVisitor {
    type Value = Option<DateTime<Utc>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "null or a datetime string")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        Ok(Some(d.deserialize_str(DateTimeFromCustomFormatVisitor)?))
    }
}

pub fn deserialize_optional_datetime<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: de::Deserializer<'de>,
{
    d.deserialize_option(OptionalDateTimeFromCustomFormatVisitor)
}

struct DateTimeFromCustomFormatVisitor;

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";


pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: de::Deserializer<'de>,
{
    d.deserialize_str(DateTimeFromCustomFormatVisitor)
}

impl<'de> de::Visitor<'de> for DateTimeFromCustomFormatVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a datetime string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        match NaiveDateTime::parse_from_str(value, FORMAT) {
            Ok(ndt) => Ok(DateTime::from_utc(ndt, Utc)),
            Err(e) => Err(E::custom(format!("Parse error {} for {}", e, value))),
        }
    }
}
