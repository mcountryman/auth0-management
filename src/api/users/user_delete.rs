//! Delete a user.

use crate::impl_request;

/// Delete a user.
///
/// # Scopes
/// * `delete:users`
pub struct UserDelete(String);

impl UserDelete {
  /// Create delete user request.
  pub fn new<S: AsRef<str>>(id: S) -> Self {
    Self(id.as_ref().to_string())
  }
}

impl_request!(UserDelete, |req, driver| {
  driver.build(Method::DELETE, &format!("api/v2/users/{}", req.0))
});
