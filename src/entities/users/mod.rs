//! User management.

pub mod user;
pub mod user_create;

use crate::{
  error::{Auth0Error, Auth0Result},
  Auth0,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub use user::*;
pub use user_create::*;

/// Contains methods to access the `/users` endpoint.
pub struct UsersClient(Arc<Auth0>);

impl UsersClient {
  /// Assigns roles to a user.
  pub async fn assign_roles(&self, id: &str, request: i32) -> Auth0Result<()> {
    self
      .0
      .post(format!("users/{id}/roles"))?
      .json(&request)
      .send()
      .await?
      .error_for_status()?;

    Ok(())
  }

  /// Creates a new user.
  pub async fn create<R, A, U>(&self, request: R) -> Auth0Result<User<A, U>>
  where
    R: TryInto<UserCreate<A, U>>,
    R::Error: Into<Auth0Error>,
    A: Serialize + for<'de> Deserialize<'de>,
    U: Serialize + for<'de> Deserialize<'de>,
  {
    let request = request.try_into();
    let request = match request {
      Ok(request) => request,
      Err(err) => return Err(err.into()),
    };

    Ok(
      self
        .0
        .post("users")?
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  async fn test(users: &UsersClient) {
    users
      .create(UserCreateBuilder::<(), ()>::default().build().unwrap())
      .await
      .unwrap();
  }
}
