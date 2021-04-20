//! Retrieve details of users.
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{Auth0Client, Auth0Result, User};
use crate::{Page, Sort};

/// Retrieve details of users.
#[derive(Serialize)]
pub struct UsersFind<'a> {
  #[serde(skip_serializing)]
  client: &'a Auth0Client,

  #[serde(flatten)]
  page: Page,
  #[serde(skip_serializing_if = "Sort::is_emtpy")]
  sort: Sort,
}

impl<'a> UsersFind<'a> {
  /// Create find users request.
  pub fn new(client: &'a Auth0Client) -> Self {
    Self {
      client,

      page: Default::default(),
      sort: Default::default(),
    }
  }
}

impl<'a> AsMut<Page> for UsersFind<'a> {
  fn as_mut(&mut self) -> &mut Page {
    &mut self.page
  }
}

impl<'a> AsMut<Sort> for UsersFind<'a> {
  fn as_mut(&mut self) -> &mut Sort {
    &mut self.sort
  }
}

impl<'a> UsersFind<'a> {
  /// Send
  pub async fn send<AOut, UOut>(&self) -> Auth0Result<Vec<User<AOut, UOut>>>
  where
    AOut: DeserializeOwned,
    UOut: DeserializeOwned,
  {
    self
      .client
      .send(self.client.begin(Method::GET, "api/v2/users").query(self))
      .await
  }
}
