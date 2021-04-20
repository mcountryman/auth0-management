//! Role.
use serde::Deserialize;

/// Role.
#[derive(Clone, Debug, Deserialize)]
pub struct Role {
  /// The role id.
  pub id: String,
  /// The role name.
  pub name: String,
  /// The role description.
  pub description: String,
}
