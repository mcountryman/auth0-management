//! Create a role.

use super::role::Role;
use crate::impl_request;

/// Create role request.
#[derive(Builder, Serialize)]
#[builder(setter(into))]
pub struct RoleCreate<D> {
  #[serde(rename = "test")]
  name: String,
  #[builder(setter(strip_option))]
  description: Option<String>,

  #[serde(skip)]
  #[builder(setter(strip_option), default)]
  driver: Option<D>,
}

impl_request!(RoleCreate, RoleCreateBuilder, Role, |req, driver| {
  driver.build(Method::POST, "api/v2/roles").json(&req)
});

#[cfg(test)]
mod tests {
  #[tokio::test]
  async fn test_role_create() {}
}
