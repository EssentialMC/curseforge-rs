pub(crate) mod categories;
pub(crate) mod core;
pub(crate) mod file;
pub(crate) mod games;
pub(crate) mod mods;

pub use self::categories::*;
pub use self::core::*;
pub use self::file::*;
pub use self::games::*;
pub use self::mods::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

pub(crate) fn nullable_str<'de, D: Deserializer<'de>>(
    deser: D,
) -> Result<Option<String>, D::Error> {
    let maybe: Option<String> = Option::deserialize(deser)?;
    Ok(maybe.filter(|string| !string.is_empty()))
}

pub(crate) fn nullable_datetime<'de, D: Deserializer<'de>>(
    deser: D,
) -> Result<Option<DateTime<Utc>>, D::Error> {
    struct NullableDateTimeVisitor;

    impl<'de> serde::de::Visitor<'de> for NullableDateTimeVisitor {
        type Value = Option<DateTime<Utc>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str(
                "a string encoding a datetime, with '0001-01-01T00:00:00' treated as a null value",
            )
        }

        fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if string == "0001-01-01T00:00:00" {
                Ok(None)
            } else {
                Ok(Some(string.parse().map_err(E::custom)?))
            }
        }
    }

    deser.deserialize_str(NullableDateTimeVisitor)
}
