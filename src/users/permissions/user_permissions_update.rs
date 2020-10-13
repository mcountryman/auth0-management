//! Assign permissions to a user.

use crate::{Auth0RequestBuilder, Auth0};
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
pub struct UserPermissionsUpdate<'a> {
  client: &'a Auth0,
  
  id: String,
  permissions: Vec<Permission>,
}

impl<'a> UserPermissionsUpdate<'a> {
  /// Create assign user permissions request.
  ///
  /// # Arguments
  /// * `id` - The user id.
  pub fn new(client: &'a Auth0, id: &str) -> Self {
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

impl<'a> AsRef<Auth0> for UserPermissionsUpdate<'a> {
  fn as_ref(&self) -> &Auth0 {
    self.client
  }
}

impl<'a> Auth0RequestBuilder for UserPermissionsUpdate<'a> {
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

// #[cfg(test)]
// mod tests {
//   use crate::Auth0RequestBuilder;
//   use crate::{Permission, UserPermissionsUpdate};
//   use reqwest::Client;
// 
//   #[test]
//   fn test_create() {
//     let req = UserPermissionsUpdate::new("USER_ID")
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
//     assert_eq!(req.url().path(), "/api/v2/users/USER_ID/permissions");
//     assert_eq!(body.len(), 3);
//     assert_eq!(body.first().unwrap().name, "test1");
//     assert_eq!(body.last().unwrap().name, "test3");
//   }
// }
