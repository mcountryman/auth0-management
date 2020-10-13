//! Retrieve all permissions associated with the user.
use reqwest::{Method, RequestBuilder};

use crate::{Auth0Client, Auth0RequestBuilder};
use crate::{Page, Permission};

/// Provides data for get user permissions request.
///
/// # Scopes
/// * `read:users`
///
/// # Example
/// ```
/// async fn dump_permissions() {}
/// ```
pub struct UserPermissionsGet<'a> {
  client: &'a Auth0Client,

  id: String,
  page: Page,
}

impl<'a> UserPermissionsGet<'a> {
  /// Create get user permissions request.
  pub fn new(client: &'a Auth0Client, id: &str) -> Self {
    Self {
      client,

      id: id.to_owned(),
      page: Default::default(),
    }
  }
}

impl<'a> AsMut<Page> for UserPermissionsGet<'a> {
  fn as_mut(&mut self) -> &mut Page {
    &mut self.page
  }
}

impl<'a> AsRef<Auth0Client> for UserPermissionsGet<'a> {
  fn as_ref(&self) -> &Auth0Client {
    self.client
  }
}

impl<'a> Auth0RequestBuilder for UserPermissionsGet<'a> {
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
