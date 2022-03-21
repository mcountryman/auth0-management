//! Remove permissions from a user.

use reqwest::{Method, RequestBuilder};

use crate::Permission;
use crate::{Auth0Client, Auth0RequestBuilder};

/// Provides data for creating delete user permission request.
///
/// # Scopes
/// * `update:users`
pub struct UserPermissionsDelete<'a> {
  client: &'a Auth0Client,

  id: String,
  permissions: Vec<Permission>,
}

impl<'a> UserPermissionsDelete<'a> {
  /// Create permission delete request.
  ///
  /// # Arguments
  /// * `id` - The id of the user.
  pub fn new(client: &'a Auth0Client, id: &str) -> Self {
    Self {
      client,

      id: id.to_owned(),
      permissions: Vec::new(),
    }
  }

  /// Add permission to delete request.
  ///
  /// # Arguments
  /// * `permission` - The permission to delete.
  pub fn permission(&mut self, permission: Permission) -> &mut Self {
    self.permissions.push(permission);
    self
  }

  /// Add permissions to delete request.
  ///
  /// # Arguments
  /// * `permissions` - The permissions to delete.
  pub fn permissions<P: AsRef<[Permission]>>(&mut self, permissions: P) -> &mut Self {
    self.permissions.extend_from_slice(permissions.as_ref());
    self
  }
}

impl<'a> AsRef<Auth0Client> for UserPermissionsDelete<'a> {
  fn as_ref(&self) -> &Auth0Client {
    self.client
  }
}

impl<'a> Auth0RequestBuilder for UserPermissionsDelete<'a> {
  fn build(&self, client: &Auth0Client) -> RequestBuilder {
    client
      .begin(
        Method::DELETE,
        &format!("api/v2/users/{}/permissions", self.id),
      )
      .json(&self.permissions)
  }
}
