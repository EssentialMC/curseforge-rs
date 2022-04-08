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

use serde::{Deserialize, Deserializer};

pub(crate) fn nullable_str<'de, D: Deserializer<'de>>(
    deser: D,
) -> Result<Option<String>, D::Error> {
    let maybe: Option<String> = Option::deserialize(deser)?;
    Ok(maybe.filter(|string| !string.is_empty()))
}
