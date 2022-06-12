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
//! For examples on how to use this crate, see the [tests directory] in the
//! GitHub repository.
//!
//! [tests directory]: https://github.com/EssentialMC/curseforge-rs/tree/master/tests
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
#![warn(missing_docs)]

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

/// The main error type used throughout the crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// This is the most useful variant. This will be returned if the API
    /// response failed to parse either as valid JSON, or according to the
    /// policy for handling unknown fields set by the enabled Cargo features.
    /// See the crate documentation for [conditional
    /// compilation](crate#conditional-compilation).
    #[error("there was an error deserializing a response\n{error}\nencountered at:\n{uri}")]
    Deserialize {
        /// The URI that the initial request was sent to.
        uri: url::Url,
        /// The source error that this variant was constructed from.
        #[source]
        error: serde_path_to_error::Error<serde_json::Error>,
        /// The bytes the body content bytes of the response.
        bytes: Box<Vec<u8>>,
    },
    /// Sometimes the backend can throw an error, either because something was
    /// configured wrongly or an internal error such as connection loss could
    /// have happened.
    #[error("there was an error constructing or receiving a request\n{0}")]
    Request(#[from] isahc::Error),
    /// A request to a URI that was expected to return successfully with `200:
    /// OK` has failed to do so. This contains the status code that was recieved
    /// instead, and the bytes in the body of the response.
    #[error("response was expected to be status 200 OK but got {status}\nencountered at:\n{uri}")]
    StatusNotOk {
        /// The URI that the initial request was sent to.
        uri: url::Url,
        /// The response status code that was returned, not `200: OK`.
        status: isahc::http::StatusCode,
        /// The bytes the body content bytes of the response.
        bytes: Box<Vec<u8>>,
    },
    /// This variant will wrap an [`isahc::http::Error`] when configuring the
    /// client has failed to produce a stable instance of the backend.
    #[error("there was an error constructing the request\n{0}")]
    Http(#[from] isahc::http::Error),
    /// Variant specifically for when parsing the base URL fails.
    #[error("the string provided failed to parse as a URL\n{0}")]
    ParseUrl(#[from] url::ParseError),
    /// The URl that was provided cannot be used as a base.
    #[error("the URL provided cannot be a base")]
    BadBaseUrl,
}
