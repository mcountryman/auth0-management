use serde::{Deserialize, Serialize};

pub use assign::*;
pub use delete::*;
pub use get::*;

pub mod assign;
pub mod delete;
pub mod get;

/// User permission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
  /// Name of this permission.
  #[serde(rename = "permission_name")]
  pub name: String,
  /// Description of this permission.
  pub description: String,
  /// Resource server (API) name this permission is for.
  pub resource_server_name: String,
  /// Resource server (API) identifier that this permission is for.
  pub resource_server_identifier: String,
}
