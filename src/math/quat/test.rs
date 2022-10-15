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
fn test_dot_product() {
  let quat0 = Quat {
    w: 2.0,
    x: 3.0,
    y: 4.0,
    z: 5.0,
  };
  let quat1 = Quat {
    w: 2.0,
    x: 3.0,
    y: 4.0,
    z: 5.0,
  };
  let quat2 = Quat {
    w: 4.0,
    x: 9.0,
    y: 16.0,
    z: 25.0,
  };
  assert_eq!(Quat::dot_product(&quat0, &quat1), quat2);
}

#[test]
fn test_from_quat_to_axis_angle() {
  assert_eq!(AxisAngle::from(Quat::default()), AxisAngle::default());
}
