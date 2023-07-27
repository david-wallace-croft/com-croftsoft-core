// =============================================================================
//! - Unit tests for the Security module operation implementations
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2001-04-12
//! - Java updated: 2003-06-17
//! - Rust created: 2022-11-21
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.security.Authentication
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

use crate::security::structures::Authentication;

#[test]
fn test_partial_eq() {
  let auth0: Authentication = Authentication {
    password: String::from("p0"),
    username: String::from("u0"),
  };
  let auth1: Authentication = Authentication {
    password: String::from("p1"),
    username: String::from("u1"),
  };
  assert!(auth0 == auth0);
  assert!(auth1 == auth1);
  assert!(auth0 != auth1);
}
