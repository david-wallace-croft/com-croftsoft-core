// =============================================================================
//! - Unit tests for AxisAngle functions, methods, and trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-10-11
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

#[test]
fn test_matches_closely() {
  let axis_angle_0 = AxisAngle {
    radians: 0.0,
    x: 1.0,
    y: 2.0,
    z: 3.0,
  };
  let axis_angle_1 = AxisAngle {
    radians: 0.01,
    x: 1.01,
    y: 2.01,
    z: 3.01,
  };
  assert!(axis_angle_0.matches_closely(&axis_angle_1, 0.1));
  assert!(!axis_angle_0.matches_closely(&axis_angle_1, 0.001));
}

#[test]
fn test_matches_exactly() {
  let axis_angle_0 = AxisAngle {
    radians: 0.0,
    x: 1.0,
    y: 2.0,
    z: 3.0,
  };
  let axis_angle_1 = AxisAngle {
    radians: 0.01,
    x: 1.01,
    y: 2.01,
    z: 3.01,
  };
  assert!(axis_angle_0.matches_exactly(&axis_angle_0));
  assert!(!axis_angle_0.matches_exactly(&axis_angle_1));
}

#[test]
fn test_normalize() {
  let mut axis_angle = AxisAngle {
    radians: 0.0,
    x: 1.0,
    y: 1.0,
    z: 1.0,
  };
  let expected_normalized_axis_angle = AxisAngle {
    radians: 0.0,
    x: 0.5773502691896258,
    y: 0.5773502691896258,
    z: 0.5773502691896258,
  };
  assert!(axis_angle
    .normalize()
    .matches_closely(&expected_normalized_axis_angle, 0.001));
  assert!(axis_angle.matches_closely(&expected_normalized_axis_angle, 0.001));
}
