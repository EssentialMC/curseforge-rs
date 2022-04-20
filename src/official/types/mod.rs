//! This module contains strong types for all return values from the CurseForge
//! API.
//!
//! ## Handling Unknown Fields
//!
//! There are two optional features that control the behavior when deserializing
//! unknown fields, for the case when the remote API changes between releases of
//! this crate. These are `allow-unknown-fields` and `deny-unknown-fields`.
//!
//! - The `deny-unknown-fields` feature will enable Serde's
//!   `#[serde(deny_unknown_fields)]` annotation on all structures and is
//!   intended primarily for tests, to ensure that all API fields and
//!   enumeration variants are accounted for before a release.
//! - The `allow-unknown-fields` feature is intended as a stopgap solution in
//!   the situation where the latest version of this crate has not been updated
//!   for the latest changes in the API, but the API is returning data that you
//!   need access to.
//!   - All structures will have an extra field, named `other_fields`, with the
//!     type [`serde_json::Value`] that will contain the remaining values that
//!     could not be fit into the known fields of the strong type.
//!   - All enumerations will have an extra `Unknown` variant that will be used
//!     if the API responded with an unknown variant. If the type is annotated
//!     with `#[repr(u8)]`, the value of this variant will be [`u8::MAX`].
//! - The default behavior when neither of these features are enabled is to
//!   ignore any unknown fields returned from the remote API, and just
//!   deserialize what is expected. Deserializing enumeration variants will
//!   still fail if the API returns something unexpected.

pub(crate) mod categories;
pub(crate) mod core;
pub(crate) mod files;
pub(crate) mod games;
pub(crate) mod projects;

pub use self::categories::*;
pub use self::core::*;
pub use self::files::*;
pub use self::games::*;
pub use self::projects::*;

pub(crate) mod fixes {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer};

    pub fn nullable_string<'de, D: Deserializer<'de>>(
        deser: D,
    ) -> Result<Option<String>, D::Error> {
        let maybe: Option<String> = Option::deserialize(deser)?;
        Ok(maybe.filter(|string| !string.is_empty()))
    }

    pub fn nullable_datetime<'de, D: Deserializer<'de>>(
        deser: D,
    ) -> Result<Option<DateTime<Utc>>, D::Error> {
        struct NullableDateTimeVisitor;

        impl<'de> serde::de::Visitor<'de> for NullableDateTimeVisitor {
            type Value = Option<DateTime<Utc>>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "a string encoding a datetime, with '0001-01-01T00:00:00' treated as a null \
                     value",
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
}
