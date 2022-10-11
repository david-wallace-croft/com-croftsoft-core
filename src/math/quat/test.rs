// =============================================================================
//! - Unit tests for Quat functions, methods, and trait implementations
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
fn test_from_axis_angle() {
  let axis_angle = AxisAngle {
    radians: 0.0,
    x: 1.0,
    y: 1.0,
    z: 1.0,
  };
  assert_eq!(
    Quat {
      w: 1.0,
      x: 0.0,
      y: 0.0,
      z: 0.0
    },
    axis_angle.into()
  );
}
