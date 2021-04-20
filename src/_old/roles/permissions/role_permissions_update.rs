//! Assign permissions to a role.

use reqwest::{Method, RequestBuilder};

use crate::Permission;
use crate::{Auth0Client, Auth0RequestBuilder};

/// Assign role permissions.
///
/// # Scopes
/// * `update:roles`
///
/// # Example
/// ```
/// async fn add_permission() {}
/// ```
pub struct RolePermissionsUpdate<'a> {
  client: &'a Auth0Client,

  id: String,
  permissions: Vec<Permission>,
}

impl<'a> RolePermissionsUpdate<'a> {
  /// Create assign role permissions request.
  ///
  /// # Arguments
  /// * `id` - The role id.
  pub fn new(client: &'a Auth0Client, id: &str) -> Self {
    Self {
      client,

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

impl<'a> AsRef<Auth0Client> for RolePermissionsUpdate<'a> {
  fn as_ref(&self) -> &Auth0Client {
    self.client
  }
}

impl<'a> Auth0RequestBuilder for RolePermissionsUpdate<'a> {
  fn build(&self, client: &Auth0Client) -> RequestBuilder {
    client
      .begin(
        Method::POST,
        &format!("api/v2/roles/{}/permissions", self.id),
      )
      .json(&self.permissions)
  }
}

// #[cfg(test)]
// mod tests {
//   use crate::Auth0RequestBuilder;
//   use crate::{Permission, RolePermissionsUpdate};
//   use reqwest::Client;
//
//   #[test]
//   fn test_create() {
//     let req = RolePermissionsUpdate::new("USER_ID")
//       .permission(Permission {
//         name: "test1".to_string(),
//         description: "test1".to_string(),
//         resource_server_name: "test1".to_string(),
//         resource_server_identifier: "test1".to_string(),
//       })
//       .permissions(&[
//         Permission {
//           name: "test2".to_string(),
//           description: "test2".to_string(),
//           resource_server_name: "test2".to_string(),
//           resource_server_identifier: "test2".to_string(),
//         },
//         Permission {
//           name: "test3".to_string(),
//           description: "test3".to_string(),
//           resource_server_name: "test3".to_string(),
//           resource_server_identifier: "test3".to_string(),
//         },
//       ])
//       .build(|method, path| {
//         Client::new().request(method, &format!("https://ipsum/{}", path))
//       })
//       .build()
//       .unwrap();
//
//     let body: Vec<Permission> =
//       serde_json::from_reader(req.body().unwrap().as_bytes().unwrap()).unwrap();
//
//     assert_eq!(req.url().path(), "/api/v2/roles/USER_ID/permissions");
//     assert_eq!(body.len(), 3);
//     assert_eq!(body.first().unwrap().name, "test1");
//     assert_eq!(body.last().unwrap().name, "test3");
//   }
// }
