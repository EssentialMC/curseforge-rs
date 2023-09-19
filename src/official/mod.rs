//! Exports all modules regarding the official CurseForge API.
//!
//! It is recommended to use the [`prelude`].

#[doc(hidden)]
pub mod client;
pub mod endpoints;
pub mod request;
pub mod types;

pub use crate::Error;
pub use client::Client;

/// All members defined within this crate are re-exported flatly at this path
/// for convenience.
pub mod prelude {
    pub use super::client::{Client, ClientOptions};
    pub use super::endpoints as e;
    #[doc(inline)]
    pub use super::endpoints::DEFAULT_API_BASE as CF_DEFAULT_API_BASE;
    #[doc(inline)]
    pub use super::request::*;
    #[doc(inline)]
    pub use super::types::*;
}
