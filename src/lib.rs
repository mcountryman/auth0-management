#![warn(missing_docs)]
//! Unofficial Auth0 Management API.
//!
//! # Connection Handling
//! Authentication with Auth0 is handled for you provided you provide the values defined
//! in the example below.  For additional information reference the official Auth0 guide
//! for obtaining auth tokens [here](https://auth0.com/docs/tokens).
//! ```
//! use auth0_management::Auth0;
//!
//! async fn init() {
//!   let auth0 = Auth0::builder()
//!     .domain(env!("AUTH0_DOMAIN"))
//!     .audience(env!("AUTH0_AUDIENCE"))
//!     .client_id(env!("AUTH0_CLIENT_ID"))
//!     .client_secret(env!("AUTH0_CLIENT_SECRET"))
//!     .build()
//!     .unwrap();
//! }
//! ```
//! # User Management
//! ```
//! use serde::{Serialize, Deserialize};
//! use auth0_management::{Auth0, Pageable, Sortable, Ordering, Auth0Request, User};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Metadata;
//!
//! async fn test_users(auth0: &Auth0) {
//!   // Create a user.
//!   auth0
//!     .create_user()
//!     .email("test@example.test")
//!     .connection("CONNECTION_ID")
//!     .send()
//!     .await
//!     .expect("Failed to create a user.");
//!
//!   // Find first user user sort by email address.
//!   let users: Vec<User<Metadata, Metadata>> = auth0
//!     .find_users()
//!     .page(0)
//!     .per_page(1)
//!     .sort("email", Ordering::Ascending)
//!     .send()
//!     .await
//!     .expect("Failed to fetch users.");
//!
//!   // Update found user.
//!   auth0
//!     .update_user(
//!       &users
//!         .first()
//!         .expect("No users found")
//!         .user_id
//!     )
//!     .email("test@test.test")
//!     .send()
//!     .await
//!     .expect("Failed to update user.");
//! }
//! ```

#[doc(inline)]
pub use api::*;
pub use builder::*;
pub use client::*;
pub use page::*;
pub use request::*;
pub use sort::*;
pub use users::*;

use crate::client::Auth0Client;
use serde::de::DeserializeOwned;

mod request;
pub mod sort;

#[allow(missing_docs)]
pub mod api;
pub mod builder;
pub mod client;
pub mod page;
#[doc(hidden)]
pub mod rate;
#[doc(hidden)]
pub mod token;
pub mod users;

/// Auth0 management client.
pub struct Auth0(Auth0Client);

impl Auth0 {
  /// Create Auth0 client
  pub fn builder() -> Auth0Builder {
    Default::default()
  }

  /// Create a new user for a given [database](https://auth0.com/docs/connections/database)
  /// or [passwordless](https://auth0.com/docs/connections/passwordless) connection.
  ///
  /// Note: connection is required but other parameters such as email and password are
  /// dependent upon the type of connection.
  ///
  /// # Scopes
  /// * `create:users`
  pub fn create_user<A, U>(&self) -> UserCreate<'_, A, U> {
    UserCreate::new(&self.0)
  }

  /// Delete a user.
  ///
  /// # Arguments
  /// * `id` - The id of the user to delete.
  /// # Scopes
  /// * `delete:users`
  pub async fn delete_user<S: AsRef<String>>(&self, id: S) -> Result<(), Auth0Error> {
    self.0.query(&UserDelete::new(id)).await
  }

  /// Retrieve the first confirmed
  /// [Guardian](https://auth0.com/docs/multifactor-authentication/guardian) enrollment
  /// for a user.
  ///
  /// # Scopes
  /// * `read:users`
  pub async fn get_user_enrollments<S: AsRef<String>>(
    &self,
    id: S,
  ) -> Result<Vec<UserEnrollment>, Auth0Error> {
    self.0.query(&UserEnrollmentsGet::new(id)).await
  }

  /// Retrieve user details. A list of fields to include or exclude may also be specified.
  ///
  /// # Arguments
  /// * `id` - The ID of the user to retrieve.
  /// # Scopes
  /// * `read:users`
  /// * `read:user_idp_tokens`
  pub async fn get_user<A, U, S: AsRef<String>>(
    &self,
    id: S,
  ) -> Result<User<A, U>, Auth0Error>
  where
    A: DeserializeOwned,
    U: DeserializeOwned,
  {
    self.0.query(&UserGet::new(id)).await
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
  pub fn get_user_logs<S: AsRef<String>>(&self, id: S) -> UserLogsGet<'_> {
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
  pub fn update_user<A, U, S: AsRef<str>>(&self, id: S) -> UserUpdate<'_, A, U> {
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
  pub fn find_users<A, U>(&self) -> UsersFind<'_, A, U> {
    UsersFind::new(&self.0)
  }
}
