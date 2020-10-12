//! Role request builders.
use serde::Deserialize;

pub use roles_find::*;

mod roles_find;

/// Describes container of permissions applicable to a user.
#[derive(Clone, Debug, Deserialize)]
pub struct Role {
  /// The unique identifier.
  pub id: String,
  /// The name.
  pub name: String,
  /// The description.
  pub description: String,
}
