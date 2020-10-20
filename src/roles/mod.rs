//! Role request builders.
use std::sync::Arc;

use reqwest::Method;

pub use permissions::*;
pub use role::*;
pub use role_create::*;
pub use role_update::*;
pub use role_users_add::*;
pub use role_users_get::*;
pub use roles_find::*;

use crate::{Auth0Client, Auth0Result};

pub mod permissions;
pub mod role;
pub mod role_create;
pub mod role_update;
pub mod role_users_add;
pub mod role_users_get;
pub mod roles_find;

/// Roles manager.
pub struct RolesManager(Arc<Auth0Client>);

impl RolesManager {
  /// Create roles manager.
  pub fn new(client: Arc<Auth0Client>) -> Self {
    Self(client)
  }

  /// Retrieve filtered list of roles that can be assigned to users or groups.
  ///
  /// # Scopes
  /// * `read:roles`
  pub fn find(&self) -> RolesFind<'_> {
    RolesFind::new(&self.0)
  }

  /// Create a new role.
  ///
  /// # Arguments
  /// * `name` - The name of the role.
  /// # Scopes
  /// * `create:roles`
  pub fn create(&self, name: &str) -> RoleCreate<'_> {
    RoleCreate::new(&self.0, name)
  }

  /// Retrieve a role.
  ///
  /// # Arguments
  /// * `id` - The id of the role.
  /// # Scopes
  /// * `read:roles`
  pub async fn get(&self, id: &str) -> Auth0Result<Role> {
    self
      .0
      .send_simple(Method::GET, &format!("api/v2/roles/{}", id))
      .await
  }

  /// Delete a role.
  ///
  /// # Arguments
  /// * `id` - The id of the role.
  /// # Scopes
  /// * `delete:roles`
  pub async fn delete(&self, id: &str) -> Auth0Result<()> {
    self
      .0
      .send_simple(Method::DELETE, &format!("api/v2/roles/{}", id))
      .await
  }

  /// Update a role.
  ///
  /// # Arguments
  /// * `id` - The id of the role.
  /// # Scopes
  /// * `update:roles`
  pub fn update(&self, id: &str) -> RoleUpdate<'_> {
    RoleUpdate::new(&self.0, id)
  }

  /// Retrieve list of permissions granted by a role.
  ///
  /// # Arguments
  /// * `id` - The id of the role to list granted permissions.
  /// # Scopes
  /// * `read:roles`
  pub fn get_permissions(&self, id: &str) -> RolePermissionsGet<'_> {
    RolePermissionsGet::new(&self.0, id)
  }

  /// Remove permissions associated with a role.
  ///
  /// # Arguments
  /// * `id` - The id the role to remove permissions from.
  /// # Scopes
  /// * `update:roles`
  pub fn delete_permissions(&self, id: &str) -> RolePermissionsDelete<'_> {
    RolePermissionsDelete::new(&self.0, id)
  }

  /// Associate permissions with a role.
  ///
  /// # Arguments
  /// * `id` - The id of the role to add permissions to.
  /// # Scopes
  /// * `update:roles`
  pub fn update_permissions(&self, id: &str) -> RolePermissionsUpdate<'_> {
    RolePermissionsUpdate::new(&self.0, id)
  }

  /// Retrieve users associated with a role.
  ///
  /// # Arguments
  /// * `id` - The id of the role to retrieve a list of users associated with.
  /// # Scopes
  /// * `read:users`
  /// * `read:roles`
  /// * `read:role_members`
  pub fn get_users(&self, id: &str) -> RoleUsersGet<'_> {
    RoleUsersGet::new(&self.0, id)
  }

  /// Assign users to a role.
  ///
  /// # Arguments
  /// * `id` - The id of the role to assign users to.
  /// # Scopes
  /// * `update:roles`
  /// * `create:role_members`
  pub fn add_users(&self, id: &str) -> RoleUsersAdd<'_> {
    RoleUsersAdd::new(&self.0, id)
  }
}
