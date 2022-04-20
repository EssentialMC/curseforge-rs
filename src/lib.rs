#![cfg_attr(doc, feature(doc_auto_cfg))]

#[cfg(all(feature = "allow-unknown-fields", feature = "deny-unknown-fields"))]
compile_error!(
    r#"features "allow-unknown-fields" and "deny-unknown-fields" are mutually exclusive"#
);

#[doc(hidden)]
pub mod cfwidget;
pub mod official;

#[doc(inline)]
pub use official::*;
