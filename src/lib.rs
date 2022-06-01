//! # CurseForge API Wrapper for Rust
//!
//! This crate aims to provide a complete wrapper over the REST API for
//! [CurseForge Core].
//!
//! It also has secondary support for the [CFWidget] API.
//!
//! The documentation for this crate is minimal, and that is on purpose. For the
//! most up-to-date information, refer to the documentation for the REST API
//! that you are concerned with. Links to relevant sections are provided on
//! every response, parameter, and body type, as well as endpoint methods. If
//! there is anything missing, please file an issue on the repository.
//!
//! [CurseForge Core]: https://docs.curseforge.com/#getting-started
//! [CFWidget]: https://www.cfwidget.com/
//!
//! ## Conditional Compilation
//!
//! This crate has two main cargo features, `official` and `cfwidget`, that
//! enable compilation for the modules for each API respectively. There are also
//! two additional features, `allow-unknown-fields` and `deny-unknown-fields`
//! that control how unexpected responses from the APIs are handled.
//!
//! The `official` module can be disabled with `default-features = false`, and
//! the `cfwidget` module can be enabled with `features = ["cfwidget"]` in your
//! `Cargo.toml`. The feature and corresponding module `cfwidget` is not enabled
//! by default, and must be enabled explicitly in your `Cargo.toml`.
//!
//! **If only one module is enabled, its exports will be raised to the crate
//! root.**
//!
//! You may enable either one or both modules.
//!
//! For information on the other two features, `allow-unknown-fields` and
//! `deny-unknown-fields`, see the documentation on the
//! [`crate::official::types`] module.
//!
//! For examples on how to use this crate, see the [tests directory] in the
//! GitHub repository.
//!
//! [tests directory]: https://github.com/EssentialMC/curseforge-rs/tree/master/tests
//!
//! ## Generating Documentation
//!
//! The documentation is expected to be built with nightly, and certain features
//! enabled for all the information to be included. The modules visible in
//! the root will change depending on the API modules that you have enabled.
//!
//! ```shell
//! $ cargo +nightly doc --features "cfwidget" --features "allow-unknown-fields"
//! ```

#![cfg_attr(doc, feature(doc_auto_cfg))]

#[cfg(all(feature = "allow-unknown-fields", feature = "deny-unknown-fields"))]
compile_error!(
    r#"features "allow-unknown-fields" and "deny-unknown-fields" are mutually exclusive"#
);

#[cfg(feature = "cfwidget")]
#[cfg_attr(not(feature = "official"), doc(hidden))]
pub mod cfwidget;
#[cfg(feature = "official")]
#[cfg_attr(not(feature = "cfwidget"), doc(hidden))]
pub mod official;

#[cfg(all(feature = "official", not(feature = "cfwidget")))]
#[doc(inline)]
pub use official::*;

#[cfg(all(feature = "cfwidget", not(feature = "official")))]
#[doc(inline)]
pub use cfwidget::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("there was an error deserializing a response:\n{error}\nencountered at: '{uri}'")]
    Deserialize {
        uri: url::Url,
        #[source]
        error: serde_path_to_error::Error<serde_json::Error>,
        bytes: Box<Vec<u8>>,
    },
    #[error("there was an error constructing or receiving a request\n{0}")]
    Request(#[from] isahc::Error),
    #[error("request was expected to recieve 200 OK but got {status}\nencountered at: '{uri}'")]
    StatusNotOk {
        uri: url::Url,
        status: isahc::http::StatusCode,
        bytes: Box<Vec<u8>>,
    },
    #[error("there was an error constructing the request\n{0}")]
    Http(#[from] isahc::http::Error),
    #[error("the string provided failed to parse as a URL\n{0}")]
    ParseBaseUrl(#[from] url::ParseError),
    #[error("the URL provided cannot be a base")]
    BadBaseUrl,
}
