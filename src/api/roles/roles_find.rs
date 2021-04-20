//! Retrieve filtered list of roles that can be assigned to users or groups.

use reqwest::Method;
use serde::Serialize;

use crate::{Auth0Client, Auth0Result, Page, Role};

/// Retrieve filtered list of roles that can be assigned to users or groups.
#[derive(Clone, Debug, Serialize)]
pub struct RolesFind<'a> {
  #[serde(skip_serializing)]
  client: &'a Auth0Client,

  #[serde(flatten)]
  page: Page,
  filter: Option<String>,
}

impl<'a> RolesFind<'a> {
  /// Create find roles request.
  pub fn new(client: &'a Auth0Client) -> Self {
    Self {
      client,

      page: Default::default(),
      filter: None,
    }
  }

  /// Filter on rule name (case-insensitive).
  pub fn filter(&mut self, filter: &str) -> &mut Self {
    self.filter = Some(filter.to_owned());
    self
  }

  /// Send request.
  pub async fn send(&self) -> Auth0Result<Role> {
    self
      .client
      .send::<Role>(
        self
          .client //
          .begin(Method::GET, "api/v2/roles")
          .query(self),
      )
      .await
  }
}

impl<'a> AsMut<Page> for RolesFind<'a> {
  fn as_mut(&mut self) -> &mut Page {
    &mut self.page
  }
}
