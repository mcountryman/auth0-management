use crate::helpers::get_client;
use auth0_management::api::users::{User, UsersManager};
use serde::{Deserialize, Serialize};

mod helpers;

#[derive(Serialize, Deserialize, Debug)]
struct AppData;
#[derive(Serialize, Deserialize, Debug)]
struct UserData;

#[tokio::test]
async fn test_get_user() {
  let mut client = get_client();
  let user: User<AppData, UserData> = client
    .get_user("auth0|5f7a6299bbbd8200686a13e4")
    .await
    .unwrap()
    .unwrap();

  println!("user = {:?}", user);
}

// #[tokio::test]
// async fn test_find_user() {
//   let mut client = get_client();
//   let user: User<AppData, UserData> = client
//     .find_users("auth0|5f7a6299bbbd8200686a13e4")
//     .await
//     .unwrap()
//     .unwrap();
//
//   println!("user = {:?}", user);
// }
