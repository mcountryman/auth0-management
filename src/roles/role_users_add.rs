//! Assign users to a role.

use serde::Serialize;

use crate::{Auth0Client, Auth0Result};
use reqwest::Method;

/// Assign users to a role.
#[derive(Serialize)]
pub struct RoleUsersAdd<'a> {
  #[serde(skip_serializing)]
  client: &'a Auth0Client,
  #[serde(skip_serializing)]
  id: String,
  users: Vec<String>,
}

impl<'a> RoleUsersAdd<'a> {
  /// Create add users to role request.
  pub fn new(client: &'a Auth0Client, id: &str) -> Self {
    Self {
      client,

      id: id.to_owned(),
      users: Vec::new(),
    }
  }

  /// Add user_id to role.
  pub fn add_user(&mut self, user_id: &str) -> &mut Self {
    self.users.push(user_id.to_owned());
    self
  }

  /// Send request.
  pub async fn send(&self) -> Auth0Result<()> {
    self
      .client
      .send(
        self
          .client
          .begin(Method::POST, &format!("api/v2/roles/{}/users", self.id))
          .json(self),
      )
      .await
  }
}
