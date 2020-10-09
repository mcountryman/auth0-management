//! Retrieve all permissions associated with the user.
//!
//! # Scopes
//! * `read:users`
//!
//! # Example
//! ```
//! async fn dump_permissions() {}
//! ```
use std::ops::{Deref, DerefMut};

use reqwest::{Method, RequestBuilder};

use crate::request::Auth0Request;
use crate::{Page, User};

/// Provides data for get user permissions request.
pub struct GetUserPermissions {
  id: String,
  page: Page,
}

impl GetUserPermissions {
  /// Create get user permissions request.
  pub fn new(id: &str) -> Self {
    Self {
      id: id.to_owned(),
      page: Default::default(),
    }
  }
}

impl Deref for GetUserPermissions {
  type Target = Page;

  fn deref(&self) -> &Self::Target {
    &self.page
  }
}

impl DerefMut for GetUserPermissions {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.page
  }
}

impl<A, U> From<User<A, U>> for GetUserPermissions {
  fn from(user: User<A, U>) -> Self {
    Self::new(&user.user_id)
  }
}

impl<A, U> From<&User<A, U>> for GetUserPermissions {
  fn from(user: &User<A, U>) -> Self {
    Self::new(&user.user_id)
  }
}

impl Auth0Request for GetUserPermissions {
  type Response = Vec<Permission>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(
      Method::GET,
      &format!("api/v2/users/{}/permissions", self.id),
    )
    .query(&self.page)
  }
}
