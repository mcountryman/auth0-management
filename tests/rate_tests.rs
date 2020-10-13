// use auth0_management::rate::RateLimit;
// use std::time::{Duration, SystemTime};
//
// #[test]
// fn test_check_limit() {
//   let mut rate = RateLimit::new();
//
//   assert!(rate.check_limit());
//
//   rate.limit = 2;
//   rate.reset_secs = SystemTime::now() + Duration::from_secs(1000);
//   rate.remaining = 2;
//
//   assert!(rate.check_limit());
//   assert!(rate.check_limit());
//   assert!(!rate.check_limit());
//
//   rate.reset_secs = SystemTime::now();
//
//   assert!(rate.check_limit());
//   assert!(rate.check_limit());
// }
