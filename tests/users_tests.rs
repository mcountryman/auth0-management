use crate::helpers::get_client;
use auth0_management::api::users::{User, UserCreateOpts, UsersManager};
use serde::{Deserialize, Serialize};

mod helpers;

#[derive(Serialize, Deserialize, Debug)]
struct AppData;
#[derive(Serialize, Deserialize, Debug)]
struct UserData;

// #[tokio::test]
// async fn test_find_user() {
//   let mut client = get_client();
//   let users: Vec<User<AppData, UserData>> = client
//     .find_users(UsersFindOpts::new().page(0).page_size(100))
//     .await
//     .unwrap();
// }

#[tokio::test]
async fn test_create_read_update_delete_user() {
  let mut client = get_client();
  let user = client
    .create_user(
      UserCreateOpts::<AppData, UserData>::new()
        .name("test_user")
        .email("test_user@example.com")
        .given_name("Test")
        .family_name("User")
        .password("Testing1234!")
        .connection("Username-Password-Authentication"),
    )
    .await
    .unwrap();

  let mut user: User<AppData, UserData> =
    client.get_user(&user.user_id).await.unwrap().unwrap();

  let id = user.user_id.to_owned();

  user.name = "test".to_owned();

  let update_result = client.update_user(user).await;

  client.delete_user(&id).await.unwrap();

  update_result.unwrap();
}
