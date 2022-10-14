// =============================================================================
//! - Unit tests for Quat functions, methods, and trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-10-14
//! - Since: 2022-10-10
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use super::*;

#[test]
fn test_default() {
  assert_eq!(
    Quat::default(),
    Quat {
      w: 1.0,
      x: 0.0,
      y: 0.0,
      z: 0.0
    }
  );
}

#[test]
fn test_from_quat_to_axis_angle() {
  assert_eq!(AxisAngle::from(Quat::default()), AxisAngle::default());
}
