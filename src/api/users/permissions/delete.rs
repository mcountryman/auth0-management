//! Remove permissions from a user.
//!
//! # Scopes
//! * `update:users`
//!
//! # Example
//! ```
//! use auth0_management::DeleteUserPermissions;
//!
//! async fn delete_permissions() {
//!   DeleteUserPermissions::new()
//!     .permissions(Vec::new());
//! }
//! ```

use reqwest::{Method, RequestBuilder};

use crate::request::Auth0Request;
use crate::Permission;

/// Provides data for creating delete user permission request.
pub struct DeleteUserPermissions {
  id: String,
  permissions: Vec<Permission>,
}

impl DeleteUserPermissions {
  /// Create permission delete request.
  ///
  /// # Arguments
  /// * `id` - The id of the user.
  pub fn new(id: &str) -> Self {
    Self {
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

impl Auth0Request for DeleteUserPermissions {
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
