#![warn(missing_docs)]
//! Unofficial Auth0 Management API.
//!
//! # Connection Handling
//! Authentication with Auth0 is handled for you provided you provide the values defined
//! in the example below.  For additional information reference the official Auth0 guide
//! for obtaining auth tokens [here](https://auth0.com/docs/tokens).
//! ```
//! use auth0_management::Auth0;
//!
//! async fn init() {
//!   let auth0 = Auth0::builder()
//!     .domain(env!("AUTH0_DOMAIN"))
//!     .audience(env!("AUTH0_AUDIENCE"))
//!     .client_id(env!("AUTH0_CLIENT_ID"))
//!     .client_secret(env!("AUTH0_CLIENT_SECRET"))
//!     .build()
//!     .unwrap();
//! }
//! ```
//! # User Management
//! ```
//! use serde::{Serialize, Deserialize};
//! use auth0_management::{Auth0, Pageable, Sortable, Ordering, Auth0Request, User};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Metadata;
//!
//! async fn test_users(auth0: &Auth0) {
//!   // Create a user.
//!   auth0
//!     .users
//!     .create()
//!     .email("test@example.test")
//!     .connection("CONNECTION_ID")
//!     .send::<(), ()>()
//!     .await
//!     .expect("Failed to create a user.");
//!
//!   // Find first user user sort by email address.
//!   let users: Vec<User<Metadata, Metadata>> = auth0
//!     .users
//!     .find()
//!     .page(0)
//!     .per_page(1)
//!     .sort("email", Ordering::Ascending)
//!     .send()
//!     .await
//!     .expect("Failed to fetch users.");
//!
//!   // Update found user.
//!   auth0
//!     .users
//!     .update(
//!       &users
//!         .first()
//!         .expect("No users found")
//!         .user_id
//!     )
//!     .email("test@test.test")
//!     .send::<(), ()>()
//!     .await
//!     .expect("Failed to update user.");
//! }
//! ```
use std::sync::Arc;

#[doc(inline)]
pub use api::*;
pub use builder::*;
pub use client::*;
pub use error::*;
pub use page::*;
pub use request::*;
pub use roles::*;
pub use sort::*;
pub use types::*;
pub use users::*;

use crate::client::Auth0Client;

mod request;
pub mod sort;

#[allow(missing_docs)]
pub mod api;
pub mod builder;
pub mod client;
pub mod error;
pub mod page;
#[doc(hidden)]
pub mod rate;
pub mod roles;
#[doc(hidden)]
pub mod token;
pub mod types;
pub mod users;

/// Auth0 management client.
pub struct Auth0 {
  /// Users manager
  pub users: UsersManager,
}

impl Auth0 {
  /// Create auth0 management api
  pub fn new(client: Auth0Client) -> Self {
    let client = Arc::new(client);

    Self {
      users: UsersManager::new(client),
    }
  }

  /// Create Auth0 client
  pub fn builder() -> Auth0Builder {
    Default::default()
  }
}
