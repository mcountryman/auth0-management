use serde::{Deserialize, Serialize};

// use crate::helpers::get_client;
// use auth0_management::{Auth0Request, Pageable};

mod helpers;

#[derive(Serialize, Deserialize, Debug)]
struct AppData;

#[derive(Serialize, Deserialize, Debug)]
struct UserData;

#[tokio::test]
async fn test_find_user() {
  // let mut client = get_client();
  // let _users = client
  //   .find_users::<AppData, UserData>()
  //   .page(0)
  //   .send()
  //   .await
  //   .unwrap();
}
