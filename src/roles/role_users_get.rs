//! Retrieve users associated with a role.

use crate::{Auth0Client, Auth0Result, Page, User};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Retrieve users associated with a role.
#[derive(Serialize)]
pub struct RoleUsersGet<'a> {
  #[serde(skip_serializing)]
  client: &'a Auth0Client,

  id: String,
  page: Page,
}

impl<'a> RoleUsersGet<'a> {
  /// Create role get users request.
  pub fn new(client: &'a Auth0Client, id: &str) -> Self {
    Self {
      client,

      id: id.to_owned(),
      page: Default::default(),
    }
  }

  /// Send request.
  pub async fn send<A, U>(&self) -> Auth0Result<Vec<User<A, U>>>
  where
    A: DeserializeOwned,
    U: DeserializeOwned,
  {
    self
      .client
      .send(
        self
          .client
          .begin(Method::GET, &format!("api/v2/roles/{}/users", self.id))
          .query(self),
      )
      .await
  }
}

impl AsMut<Page> for RoleUsersGet<'_> {
  fn as_mut(&mut self) -> &mut Page {
    &mut self.page
  }
}
