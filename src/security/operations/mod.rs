// =============================================================================
//! - Operation implementations for the Security module
//!
//! # Metadata
//! - Copyright: &copy; 2001 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-21
//! - Rust since: 2022-11-21
//! - Java version: 2003-06-17
//! - Java since: 2001-04-12
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.security.Authentication
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
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
