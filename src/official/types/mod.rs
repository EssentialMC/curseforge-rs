pub(crate) mod categories;
pub(crate) mod core;
pub(crate) mod file;
pub(crate) mod games;
pub(crate) mod projects;

pub use self::categories::*;
pub use self::core::*;
pub use self::file::*;
pub use self::games::*;
pub use self::projects::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

/// Wraps API responses which have the single field `data`.
/// This type could be publicised if it is needed for some reason.
///
/// <https://docs.curseforge.com/#tocS_Get%20Versions%20Response>
/// <https://docs.curseforge.com/#tocS_Get%20Version%20Types%20Response>
/// <https://docs.curseforge.com/#tocS_Get%20Categories%20Response>
/// <https://docs.curseforge.com/#tocS_Get%20Game%20Response>
/// <https://docs.curseforge.com/#tocS_Get%20Mod%20Response>
/// <https://docs.curseforge.com/#tocS_Get%20Mods%20Response>
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IntermResponse<T> {
    pub data: T,
}

macro_rules! request_several_body {
    ($field:ident, $field_type:ty, $iter:expr) => {{
        use ::serde::Serialize;

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct _RequestBody {
            $field: Vec<$field_type>,
        }

        _RequestBody {
            $field: $iter.collect(),
        }
    }};
}

pub(crate) use request_several_body;

pub(crate) fn nullable_string<'de, D: Deserializer<'de>>(
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
