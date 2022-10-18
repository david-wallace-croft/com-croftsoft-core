// =============================================================================
//! - Unit tests for Quat functions, methods, and trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-10-18
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
    w: 3.0,
    x: 4.0,
    y: 5.0,
    z: 6.0,
  };
  let quat2 = Quat {
    w: 6.0,
    x: 12.0,
    y: 20.0,
    z: 30.0,
  };
  assert_eq!(Quat::dot_product(&quat0, &quat1), quat2);
}

#[allow(clippy::op_ref)]
#[test]
fn test_mul() {
  let quat_lhs = Quat {
    w: 1.0,
    x: 2.0,
    y: 3.0,
    z: 4.0,
  };
  let quat_rhs = Quat {
    w: 2.0,
    x: 3.0,
    y: 4.0,
    z: 5.0,
  };
  let quat_product = Quat {
    w: -36.0,
    x: 6.0,
    y: 12.0,
    z: 12.0,
  };
  assert_eq!(quat_lhs * quat_rhs, quat_product);
  assert_eq!(quat_lhs * &quat_rhs, quat_product);
  assert_eq!(&quat_lhs * quat_rhs, quat_product);
  assert_eq!(&quat_lhs * &quat_rhs, quat_product);
}

#[test]
fn test_mul_assign() {
  let product = Quat {
    w: -36.0,
    x: 6.0,
    y: 12.0,
    z: 12.0,
  };
  // multiplicand and multiplier
  let mut multiplicand = Quat {
    w: 1.0,
    x: 2.0,
    y: 3.0,
    z: 4.0,
  };
  let multiplier = Quat {
    w: 2.0,
    x: 3.0,
    y: 4.0,
    z: 5.0,
  };
  multiplicand *= multiplier;
  assert_eq!(multiplicand, product);
  // &multiplicand and multiplier
  let mut multiplicand = &mut Quat {
    w: 1.0,
    x: 2.0,
    y: 3.0,
    z: 4.0,
  };
  multiplicand *= multiplier;
  assert_eq!(multiplicand, &product);
  // multiplicand and &multiplier
  let mut multiplicand = Quat {
    w: 1.0,
    x: 2.0,
    y: 3.0,
    z: 4.0,
  };
  let multiplier = &Quat {
    w: 2.0,
    x: 3.0,
    y: 4.0,
    z: 5.0,
  };
  multiplicand *= multiplier;
  assert_eq!(multiplicand, product);
  // &multiplicand and &multiplier
  let mut multiplicand = &mut Quat {
    w: 1.0,
    x: 2.0,
    y: 3.0,
    z: 4.0,
  };
  multiplicand *= multiplier;
  assert_eq!(multiplicand, &product);
}

#[test]
fn test_multiply_quat_with_quat() {
  let quat0 = Quat {
    w: 2.0,
    x: 3.0,
    y: 4.0,
    z: 5.0,
  };
  let quat1 = Quat {
    w: 3.0,
    x: 4.0,
    y: 5.0,
    z: 6.0,
  };
  let quat2 = Quat {
    w: -56.0,
    x: 16.0,
    y: 24.0,
    z: 26.0,
  };
  assert_eq!(Quat::multiply_quat_with_quat(&quat0, &quat1), quat2);
}

#[test]
fn test_from_quat_to_axis_angle() {
  assert_eq!(AxisAngle::from(Quat::default()), AxisAngle::default());
}
