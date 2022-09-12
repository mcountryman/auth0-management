// use serde::{Deserialize, Serialize};

// use auth0_management::{Ordering, Pageable, Sortable};

// use crate::helpers::get_client;

// mod helpers;

// #[derive(Serialize, Deserialize, Debug)]
// struct Metadata;

// #[tokio::test]
// async fn test_find_user() {
//   let auth0 = get_client();

//   // Create a user.
//   let user = auth0
//     .users
//     .create()
//     .email("test@example.test")
//     .password("Th!5!s4P445w3rd")
//     .connection("Username-Password-Authentication")
//     .send::<Metadata, Metadata>()
//     .await
//     .expect("Failed to create a user.");

//   // Find first user user sort by email address.
//   let _users = auth0
//     .users
//     .find()
//     .page(0)
//     .sort("email", Ordering::Ascending)
//     .send::<Metadata, Metadata>()
//     .await
//     .expect("Failed to fetch users.");

//   // Update found user.
//   auth0
//     .users
//     .update(&user.user_id)
//     .email("test@test.test")
//     .send::<Metadata, Metadata>()
//     .await
//     .expect("Failed to update user.");

//   auth0
//     .users
//     .delete(&user.user_id)
//     .await
//     .expect("Failed to delete user.");
// }
