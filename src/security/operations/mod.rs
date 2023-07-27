// =============================================================================
//! - Operation implementations for the Security module
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

use super::structures::Authentication;

#[cfg(test)]
mod test;

impl PartialEq for Authentication {
  fn eq(
    &self,
    other: &Self,
  ) -> bool {
    self.password == other.password && self.username == other.username
  }
}
