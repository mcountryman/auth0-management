use crate::client::Client;

pub mod api;
pub mod client;
pub mod rate;
pub mod token;

pub struct ManagementClient {
  client: Client,
}
