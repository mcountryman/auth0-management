//! Retrieve all permissions associated with the user.
use std::ops::{Deref, DerefMut};

use reqwest::{Method, RequestBuilder};

use crate::RelativeRequestBuilder;
use crate::{Page, Permission, User};

/// Provides data for get user permissions request.
///
/// # Scopes
/// * `read:users`
///
/// # Example
/// ```
/// async fn dump_permissions() {}
/// ```
pub struct UserPermissionsGet {
  id: String,
  page: Page,
}

impl UserPermissionsGet {
  /// Create get user permissions request.
  pub fn new(id: &str) -> Self {
    Self {
      id: id.to_owned(),
      page: Default::default(),
    }
  }
}

impl Deref for UserPermissionsGet {
  type Target = Page;

  fn deref(&self) -> &Self::Target {
    &self.page
  }
}

impl DerefMut for UserPermissionsGet {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.page
  }
}

impl<A, U> From<User<A, U>> for UserPermissionsGet {
  fn from(user: User<A, U>) -> Self {
    Self::new(&user.user_id)
  }
}

impl<A, U> From<&User<A, U>> for UserPermissionsGet {
  fn from(user: &User<A, U>) -> Self {
    Self::new(&user.user_id)
  }
}

impl RelativeRequestBuilder for UserPermissionsGet {
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
