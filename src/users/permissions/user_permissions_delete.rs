//! Remove permissions from a user.

use reqwest::{Method, RequestBuilder};

use crate::{Auth0RequestBuilder, Auth0};
use crate::Permission;

/// Provides data for creating delete user permission request.
///
/// # Scopes
/// * `update:users`
///
/// # Example
/// ```
/// use auth0_management::UserPermissionsDelete;
///
/// async fn delete_permissions() {
///   UserPermissionsDelete::new("USER_ID")
///     .permissions(Vec::new());
/// }
/// ```
pub struct UserPermissionsDelete<'a> {
  client: &'a Auth0,
  
  id: String,
  permissions: Vec<Permission>,
}

impl<'a> UserPermissionsDelete<'a> {
  /// Create permission delete request.
  ///
  /// # Arguments
  /// * `id` - The id of the user.
  pub fn new(client: &'a Auth0, id: &str) -> Self {
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

impl<'a> AsRef<Auth0> for UserPermissionsDelete<'a> {
  fn as_ref(&self) -> &Auth0 {
    self.client
  }
}

impl<'a> Auth0RequestBuilder for UserPermissionsDelete<'a> {
  type Response = ();

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(
      Method::DELETE,
      &format!("api/v2/users/{}/permissions", self.id),
    )
    .json(&self.permissions)
  }
}
