//! The crate root simply re-exports all members from [`official`].
//!
//! Instead of keeping all modules for the official API flat in the crate root,
//! they are in a separate module and then re-exported so that [CFWidget] can be
//! supported in the future, albeit in its own namespace.
//!
//! For examples on how to use this API, see the [tests] directory in the
//! repository.
//!
//! [CFWidget]: https://www.cfwidget.com/
//! [tests]: https://github.com/EssentialMC/curseforge-rs/tree/master/tests

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
