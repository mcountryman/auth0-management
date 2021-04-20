//! Remove permissions from a role.

use reqwest::{Method, RequestBuilder};

use crate::Permission;
use crate::{Auth0Client, Auth0RequestBuilder};

/// Provides data for creating delete role permission request.
///
/// # Scopes
/// * `update:roles`
pub struct RolePermissionsDelete<'a> {
  client: &'a Auth0Client,

  id: String,
  permissions: Vec<Permission>,
}

impl<'a> RolePermissionsDelete<'a> {
  /// Create permission delete request.
  ///
  /// # Arguments
  /// * `id` - The id of the role.
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

impl<'a> AsRef<Auth0Client> for RolePermissionsDelete<'a> {
  fn as_ref(&self) -> &Auth0Client {
    self.client
  }
}

impl<'a> Auth0RequestBuilder for RolePermissionsDelete<'a> {
  fn build(&self, client: &Auth0Client) -> RequestBuilder {
    client
      .begin(
        Method::DELETE,
        &format!("api/v2/roles/{}/permissions", self.id),
      )
      .json(&self.permissions)
  }
}
