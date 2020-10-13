#![warn(missing_docs)]
//! Unofficial Auth0 Management API

#[doc(inline)]
pub use api::*;
pub use builder::*;
pub use client::*;
pub use page::*;
pub use request::*;
pub use sort::*;
pub use users::*;

use crate::client::Auth0Client;

mod request;
pub mod sort;

#[allow(missing_docs)]
pub mod api;
pub mod builder;
pub mod client;
pub mod page;
#[doc(hidden)]
pub mod rate;
#[doc(hidden)]
pub mod token;
pub mod users;

/// Auth0 management client.
pub struct Auth0(Auth0Client);

impl Auth0 {
  /// Create Auth0 client
  pub fn builder() -> Auth0Builder {
    Default::default()
  }
}
