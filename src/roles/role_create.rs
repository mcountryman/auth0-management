//! Create a role.

use reqwest::Method;
use serde::Serialize;

use crate::{Auth0Client, Auth0Result, Role};

/// Create role request.
#[derive(Serialize)]
pub struct RoleCreate<'a> {
  #[serde(skip_serializing)]
  client: &'a Auth0Client,

  name: String,
  description: Option<String>,
}

impl<'a> RoleCreate<'a> {
  /// Create role create request.
  pub fn new(client: &'a Auth0Client, name: &str) -> Self {
    Self {
      client,

      name: name.to_owned(),
      description: None,
    }
  }

  /// Description of the role.
  pub fn description(&mut self, description: &str) -> &mut Self {
    self.description = Some(description.to_owned());
    self
  }

  /// Send request.
  pub async fn send(&self) -> Auth0Result<Role> {
    self
      .client
      .send(self.client.begin(Method::POST, "api/v2/roles").json(self))
      .await
  }
}
