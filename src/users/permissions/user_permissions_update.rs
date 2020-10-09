//! Assign permissions to a user.

use crate::request::Auth0Request;
use crate::Permission;
use reqwest::{Method, RequestBuilder};

/// Assign user permissions.
///
/// # Scopes
/// * `update:users`
///
/// # Example
/// ```
/// async fn add_permission() {}
/// ```
pub struct UserPermissionsUpdate {
  id: String,
  permissions: Vec<Permission>,
}

impl UserPermissionsUpdate {
  /// Create assign user permissions request.
  ///
  /// # Arguments
  /// * `id` - The user id.
  pub fn new(id: &str) -> Self {
    Self {
      id: id.to_owned(),
      permissions: Vec::new(),
    }
  }

  /// Add permission to create request.
  ///
  /// # Arguments
  /// * `permission` - The permission to add.
  pub fn permission(&mut self, permission: Permission) -> &mut Self {
    self.permissions.push(permission);
    self
  }

  /// Add multiple permissions to create request.
  ///
  /// # Arguments
  /// * `permissions` - The permissions to add.
  pub fn permissions<P: AsRef<[Permission]>>(&mut self, permissions: P) -> &mut Self {
    self.permissions.extend_from_slice(permissions.as_ref());
    self
  }
}

impl Auth0Request for UserPermissionsUpdate {
  type Response = ();

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(
      Method::POST,
      &format!("api/v2/users/{}/permissions", self.id),
    )
    .json(&self.permissions)
  }
}

#[cfg(test)]
mod tests {
  use crate::request::Auth0Request;
  use crate::{UserPermissionsUpdate, Permission};
  use reqwest::Client;

  #[test]
  fn test_create() {
    let req = UserPermissionsUpdate::new("USER_ID")
      .permission(Permission {
        name: "test1".to_string(),
        description: "test1".to_string(),
        resource_server_name: "test1".to_string(),
        resource_server_identifier: "test1".to_string(),
      })
      .permissions(&[
        Permission {
          name: "test2".to_string(),
          description: "test2".to_string(),
          resource_server_name: "test2".to_string(),
          resource_server_identifier: "test2".to_string(),
        },
        Permission {
          name: "test3".to_string(),
          description: "test3".to_string(),
          resource_server_name: "test3".to_string(),
          resource_server_identifier: "test3".to_string(),
        },
      ])
      .build(|method, path| {
        Client::new().request(method, &format!("https://ipsum/{}", path))
      })
      .build()
      .unwrap();

    let body: Vec<Permission> =
      serde_json::from_reader(req.body().unwrap().as_bytes().unwrap()).unwrap();

    assert_eq!(req.url().path(), "/api/v2/users/USER_ID/permissions");
    assert_eq!(body.len(), 3);
    assert_eq!(body.first().unwrap().name, "test1");
    assert_eq!(body.last().unwrap().name, "test3");
  }
}
