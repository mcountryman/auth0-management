use serde::{Deserialize, Serialize};

use auth0_management::{
  GetUserEnrollments, Ordering, PagedBuilder, UserLogsGet, UserPermissionsGet, UsersFind,
};

use crate::helpers::get_client;

mod helpers;

#[derive(Serialize, Deserialize, Debug)]
struct AppData;

#[derive(Serialize, Deserialize, Debug)]
struct UserData;

#[tokio::test]
async fn test_find_user() {
  let mut client = get_client();
  let users = client
    .query(
      UsersFind::<AppData, UserData>::new() //
        .sort("username", Ordering::Ascending)
        .page(0)
        .per_page(10),
    )
    .await
    .unwrap();

  let logs = client
    .query(UserLogsGet::from(users.first().unwrap()).per_page(100))
    .await
    .unwrap();

  let enrollments = client
    .query(&GetUserEnrollments::from(users.first().unwrap()))
    .await
    .unwrap();

  let permissions = client
    .query(&UserPermissionsGet::from(users.first().unwrap()))
    .await
    .unwrap();

  for p in permissions {
    println!("{:?}", p)
  }
}

// #[tokio::test]
// async fn test_create_read_update_delete_user() {
//   let mut client = get_client();
//   let user = client
//     .create_user(
//       UserCreateOpts::<AppData, UserData>::new()
//         .name("test_user")
//         .email("test_user@example.com")
//         .given_name("Test")
//         .family_name("User")
//         .password("Testing1234!")
//         .connection("Username-Password-Authentication"),
//     )
//     .await
//     .unwrap();
//
//   let mut user: User<AppData, UserData> =
//     client.get_user(&user.user_id).await.unwrap().unwrap();
//
//   let id = user.user_id.to_owned();
//
//   user.name = "test".to_owned();
//
//   let update_result = client.update_user(user).await;
//
//   client.delete_user(&id).await.unwrap();
//
//   update_result.unwrap();
// }
