//! Retrieve filtered list of roles that can be assigned to users or groups.

use reqwest::{Method, RequestBuilder};
use serde::Serialize;

use crate::{Page, RelativeRequestBuilder, Role};

/// Retrieve filtered list of roles that can be assigned to users or groups.
#[derive(Clone, Debug, Serialize)]
pub struct RolesFind {
  #[serde(flatten)]
  page: Page,
  filter: String,
}

impl RolesFind {
  /// Create find roles request.
  pub fn new(filter: &str) -> Self {
    Self {
      page: Default::default(),
      filter: filter.to_owned(),
    }
  }
}

impl AsMut<Page> for RolesFind {
  fn as_mut(&mut self) -> &mut Page {
    &mut self.page
  }
}

impl RelativeRequestBuilder for RolesFind {
  type Response = Vec<Role>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::GET, "/api/v2/roles").query(self)
  }
}
