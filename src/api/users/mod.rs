#![warn(missing_docs)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub use create::*;
pub use delete::*;
pub use find::*;
pub use get::*;
pub use update::*;

pub mod create;
pub mod delete;
pub mod find;
pub mod get;
pub mod update;

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
  provider: String,
  #[serde(rename = "isSocial")]
  is_social: bool,
  connection: String,
}

#[derive(Deserialize, Debug)]
pub struct User<AppMetadata = EmptyAppMetadata, UserMetadata = EmptyUserMetadata> {
  /// ID of the user which can be used when interacting with other APIs.
  pub user_id: String,
  /// Email address of this user.
  pub email: String,
  /// Whether this email address is verified (true) or unverified (false).
  pub email_verified: bool,
  /// Username of this user.
  pub username: Option<String>,
  /// Phone number for this user when using SMS connections.
  /// [Follows the E.164 recommendation.](https://en.wikipedia.org/wiki/E.164)
  pub phone_number: Option<String>,
  /// Whether this phone number has been verified (true) or not (false).
  #[serde(default)]
  pub phone_verified: bool,
  /// Date and time when this user was created.
  pub created_at: DateTime<Utc>,
  /// Date and time when this user was last updated/modified.
  pub updated_at: DateTime<Utc>,
  /// Array of user identity objects when accounts are linked.
  pub identities: Vec<Identity>,
  /// URL to picture, photo, or avatar of this user.
  pub picture: String,
  /// Name of this user.
  pub name: String,
  /// Preferred nickname or alias of this user.
  pub nickname: String,

  /// List of multi-factor authentication providers with which this user has enrolled.
  pub multifactor: Vec<String>,
  /// Last IP address from which this user logged in.
  pub last_ip: Option<String>,
  /// Last date and time this user logged in.
  pub last_login: Option<DateTime<Utc>>,
  /// Total number of logins this user has performed.
  #[serde(default)]
  pub logins_count: u32,
  /// Whether this user was blocked by an administrator (true) or is not (false).
  #[serde(default)]
  pub blocked: bool,
  /// Given name/first name/forename of this user.
  pub given_name: Option<String>,
  /// Family name/last name/surname of this user.
  pub family_name: Option<String>,
  /// User metadata to which this user has read-only access.
  pub app_metadata: Option<AppMetadata>,
  /// User metadata to which this user has read/write access.
  pub user_metadata: Option<UserMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyAppMetadata;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyUserMetadata;

//
// impl<AppMetadata, UserMetadata> Into<UserUpdateOpts<AppMetadata, UserMetadata>>
//   for User<AppMetadata, UserMetadata>
// {
//   fn into(self) -> UserUpdateOpts<AppMetadata, UserMetadata> {
//     UserUpdateOpts {
//       user_id: self.user_id,
//       blocked: Some(self.blocked),
//       email: Some(self.email),
//       phone_number: self.phone_number,
//       given_name: self.given_name,
//       family_name: self.family_name,
//       name: Some(self.name),
//       nickname: Some(self.nickname),
//       picture: Some(self.picture),
//       verify_email: None,
//       verify_phone_number: None,
//       app_metadata: None,
//       user_metadata: None,
//     }
//   }
// }
