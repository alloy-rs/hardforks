#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![no_std]

extern crate alloc;

/// Re-exported EIP-2124 forkid types.
pub use alloy_eip2124::*;

mod forkcondition;
pub use forkcondition::*;

mod hardfork;
pub use hardfork::*;
mod constants;
pub use constants::*;
// Not public API.
#[doc(hidden)]
pub mod __private {
    pub use alloc::{format, string::String};
}
