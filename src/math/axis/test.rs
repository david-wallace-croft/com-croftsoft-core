// =============================================================================
//! - Unit tests for AxisAngle functions, methods, and trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-10-10
//! - Since: 2022-10-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use super::*;

#[test]
fn test_magnitude() {
  assert!(
    AxisAngle {
      radians: 0.0,
      x: -1.0,
      y: 1.0,
      z: -1.0
    }
    .magnitude()
      - 1.7320508075688772
      < 0.001
  );
}
