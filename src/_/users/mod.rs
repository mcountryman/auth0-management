#![warn(missing_docs)]
//! User request builders.

#[doc(inline)]
pub use permissions::*;
#[doc(inline)]
pub use user::*;
#[doc(inline)]
pub use user_create::*;
#[doc(inline)]
pub use user_delete::*;
#[doc(inline)]
pub use user_enrollments_get::*;
#[doc(inline)]
pub use user_get::*;
#[doc(inline)]
pub use user_logs_get::*;
#[doc(inline)]
pub use user_update::*;
#[doc(inline)]
pub use users_find::*;

use crate::{Auth0Client, Auth0Error, Auth0RequestSimple, Auth0Result};
use serde::de::DeserializeOwned;
use std::sync::Arc;

pub mod permissions;
pub mod user;
pub mod user_create;
pub mod user_delete;
pub mod user_enrollments_get;
pub mod user_get;
pub mod user_logs_get;
pub mod user_update;
pub mod users_find;

/// Users manager
pub struct UsersManager(Arc<Auth0Client>);

impl UsersManager {
  /// Create users manager
  pub fn new(client: Arc<Auth0Client>) -> Self {
    Self(client)
  }

  /// Create a new user for a given [database](https://auth0.com/docs/connections/database)
  /// or [passwordless](https://auth0.com/docs/connections/passwordless) connection.
  ///
  /// Note: connection is required but other parameters such as email and password are
  /// dependent upon the type of connection.
  ///
  /// # Scopes
  /// * `create:users`
  pub fn create(&self) -> UserCreate<'_, (), ()> {
    UserCreate::new(&self.0)
  }

  /// Delete a user.
  ///
  /// # Arguments
  /// * `id` - The id of the user to delete.
  /// # Scopes
  /// * `delete:users`
  pub async fn delete<S: AsRef<str>>(&self, id: S) -> Auth0Result<()> {
    UserDelete::new(id).send_to(&self.0).await
  }

  /// Retrieve the first confirmed
  /// [Guardian](https://auth0.com/docs/multifactor-authentication/guardian) enrollment
  /// for a user.
  ///
  /// # Scopes
  /// * `read:users`
  pub async fn get_enrollments<S: AsRef<str>>(
    &self,
    id: S,
  ) -> Auth0Result<Vec<UserEnrollment>> {
    UserEnrollmentsGet::new(id).send_to(&self.0).await
  }

  /// Retrieve user details. A list of fields to include or exclude may also be specified.
  ///
  /// # Arguments
  /// * `id` - The ID of the user to retrieve.
  /// # Scopes
  /// * `read:users`
  /// * `read:user_idp_tokens`
  pub async fn get<S: AsRef<str>>(&self, id: S) -> Result<User<A, U>, Auth0Error>
  where
    A: DeserializeOwned + Send + Sync,
    U: DeserializeOwned + Send + Sync,
  {
    UserGet::new(id).send_to(&self.0).await
  }

  /// Retrieve log events for a specific user.
  ///
  /// Note: For more information on all possible event types, their respective acronyms
  /// and descriptions, see
  /// [Log Data Event Listing](https://auth0.com/docs/logs#log-data-event-listing).
  ///
  /// For more information on the list of fields that can be used in `sort`, see
  /// [Searchable Fields](https://auth0.com/docs/logs/query-syntax#searchable-fields).
  ///
  /// Auth0 [limits the number of logs](https://auth0.com/docs/logs#limitations) you can
  /// return by search criteria to 100 logs per request. Furthermore, you may only
  /// paginate through up to 1,000 search results. If you exceed this threshold, please
  /// redefine your search.
  ///
  /// # Scopes
  /// * `read:logs`
  /// * `read:logs_users`
  pub fn get_logs<S: AsRef<str>>(&self, id: S) -> UserLogsGet<'_> {
    UserLogsGet::new(&self.0, id)
  }

  /// Update a user.
  /// Some considerations:
  ///
  /// * The properties of the new object will replace the old ones.
  /// * The metadata fields are an exception to this rule (`user_metadata` and
  /// `app_metadata`). These properties are merged instead of being replaced but be
  /// careful, the merge only occurs on the first level.
  /// * If you are updating `email`, `email_verified`, `phone_number`, `phone_verified`,
  /// `username` or `password` of a secondary identity, you need to specify the connection
  /// property too.
  /// * If you are updating `email` or `phone_number` you can specify, optionally, the
  /// `client_id` property.
  /// * Updating `email_verified` is not supported for enterprise and passwordless sms
  /// connections.
  /// * Updating the `blocked` to `false` does not affect the user's blocked state from an
  /// excessive amount of incorrectly provided credentials. Use the "Unblock a user"
  /// endpoint from the "User Blocks" API to change the user's state.
  ///
  /// # Scopes
  /// * `update:users`
  /// * `update:users_app_metadata`
  pub fn update<S: AsRef<str>>(&self, id: S) -> UserUpdate<'_, (), ()> {
    UserUpdate::new(&self.0, id)
  }

  /// Retrieve details of users.
  ///
  /// It is possible to:
  /// - Specify a search criteria for users
  /// - Sort the users to be returned
  /// - Specify the number of users to retrieve per page and the page index
  ///
  /// The q query parameter can be used to get users that match the specified criteria
  /// using [query string syntax](https://auth0.com/docs/users/search/v3/query-syntax).
  ///
  /// [Learn more about searching for users.](https://auth0.com/docs/users/search/v3)
  ///
  /// Read about [best practices](https://auth0.com/docs/users/search/best-practices) when
  /// working with the API endpoints for retrieving users.
  ///
  /// Auth0 limits the number of users you can return. If you exceed this threshold,
  /// please redefine your search, use the
  /// [export job](https://auth0.com/docs/api/management/v2#!/Jobs/post_users_exports), or
  /// the [User Import / Export](https://auth0.com/docs/extensions/user-import-export)
  /// extension.
  ///
  /// # Scopes
  /// * `read:users`
  /// * `read:user_idp_tokens`
  pub fn find(&self) -> UsersFind<'_> {
    UsersFind::new(&self.0)
  }
}
