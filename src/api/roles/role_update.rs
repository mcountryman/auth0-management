//! Update a role.

use crate::{Auth0Client, Auth0Result, Role};
use reqwest::Method;
use serde::Serialize;

/// Update a role.
#[derive(Serialize)]
pub struct RoleUpdate<'a> {
  #[serde(skip_serializing)]
  client: &'a Auth0Client,

  id: String,
  name: Option<String>,
  description: Option<String>,
}

impl<'a> RoleUpdate<'a> {
  /// Create update role request.
  pub fn new(client: &'a Auth0Client, id: &str) -> Self {
    Self {
      client,

      id: id.to_owned(),
      name: None,
      description: None,
    }
  }

  /// The name of the role.
  pub fn name(&mut self, name: &str) -> &mut Self {
    self.name = Some(name.to_owned());
    self
  }

  /// The description of the role.
  pub fn description(&mut self, description: &str) -> &mut Self {
    self.description = Some(description.to_owned());
    self
  }

  /// Send request.
  pub async fn send(&self) -> Auth0Result<Role> {
    self
      .client
      .send(
        self
          .client
          .begin(Method::PATCH, &format!("api/v2/roles/{}", self.id))
          .json(self),
      )
      .await
  }
}
