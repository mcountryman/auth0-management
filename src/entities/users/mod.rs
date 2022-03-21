//! User management.

pub mod user;
pub mod user_create;

pub use user::*;
pub use user_create::*;

use std::sync::Arc;
use user_create::UserCreateBuilder;

/// Provides methods for creating requests that manage users.
pub struct Users<Client> {
  client: Arc<Client>,
}

impl<Client: Clone> Users<Client> {
  /// Creates a new [Users] instance.
  pub fn new<C>(client: C) -> Self
  where
    C: Into<Arc<Client>>,
  {
    Self {
      client: client.into(),
    }
  }

  /// Creates a new [UserCreate] request.
  pub fn create(&self) -> UserCreateBuilder<Client, (), ()> {
    UserCreateBuilder::default()
      .client(self.client.clone())
      .clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create() {
    let req = Users::new(())
      .create()
      .name("John Doe")
      .email("john.doe@auth0.com")
      .build()
      .unwrap();

    assert_eq!(req.name.as_deref(), Some("John Doe"));
    assert_eq!(req.email.as_deref(), Some("john.doe@auth0.com"));
  }
}
