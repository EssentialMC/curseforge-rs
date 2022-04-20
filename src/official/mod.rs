//! Exports all modules regarding the official CurseForge API.

pub mod client;
pub mod request;
pub mod types;

/// All members defined within this crate are re-exported flatly at this path
/// for convenience.
pub mod prelude {
    #[doc(inline)]
    pub use super::client::*;
    #[doc(inline)]
    pub use super::request::*;
    #[doc(inline)]
    pub use super::types::*;
}
