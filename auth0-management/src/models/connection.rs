use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Connection {
  /// The connection's identifier.
  id: String,
  /// The name of the connection.
  name: String,
  /// The type of the connection, related to the identity provider.
  strategy: String,
  /// Connection name used in login screen.
  display_name: String,

  options: HashMap<String, String>,
}
