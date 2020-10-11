//! User permission request builders.
use serde::{Deserialize, Serialize};

pub use user_permissions_delete::*;
pub use user_permissions_get::*;
pub use user_permissions_update::*;

pub mod user_permissions_delete;
pub mod user_permissions_get;
pub mod user_permissions_update;

/// Permission.
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
