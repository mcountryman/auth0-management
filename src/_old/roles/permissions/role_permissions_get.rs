//! Retrieve all permissions associated with the role.
use reqwest::{Method, RequestBuilder};

use crate::Page;
use crate::{Auth0Client, Auth0RequestBuilder};

/// Provides data for get role permissions request.
///
/// # Scopes
/// * `read:roles`
///
/// # Example
/// ```
/// async fn dump_permissions() {}
/// ```
pub struct RolePermissionsGet<'a> {
  client: &'a Auth0Client,

  id: String,
  page: Page,
}

impl<'a> RolePermissionsGet<'a> {
  /// Create get role permissions request.
  pub fn new(client: &'a Auth0Client, id: &str) -> Self {
    Self {
      client,

      id: id.to_owned(),
      page: Default::default(),
    }
  }
}

impl<'a> AsMut<Page> for RolePermissionsGet<'a> {
  fn as_mut(&mut self) -> &mut Page {
    &mut self.page
  }
}

impl<'a> AsRef<Auth0Client> for RolePermissionsGet<'a> {
  fn as_ref(&self) -> &Auth0Client {
    self.client
  }
}

impl<'a> Auth0RequestBuilder for RolePermissionsGet<'a> {
  fn build(&self, client: &Auth0Client) -> RequestBuilder {
    client
      .begin(
        Method::GET,
        &format!("api/v2/roles/{}/permissions", self.id),
      )
      .query(&self.page)
  }
}
