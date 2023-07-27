// =============================================================================
//! - Quaternion
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2008-05-02
//! - Java updated: 2008-05-09
//! - Rust created: 2022-10-10
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the Java package com.croftsoft.core.math.quat
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

use super::{
  axis::AxisAngle,
  matrix::structures::{RotationDegrees, RotationRadians},
};
use std::ops::{Mul, MulAssign};

#[cfg(test)]
mod test;

// Structures ------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
  pub w: f64,
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

// Associated Functions --------------------------------------------------------

impl Quat {
  pub fn dot_product(
    quat0: &Self,
    quat1: &Self,
  ) -> Self {
    Quat {
      w: quat0.w * quat1.w,
      x: quat0.x * quat1.x,
      y: quat0.y * quat1.y,
      z: quat0.z * quat1.z,
    }
  }

  pub fn multiply_quat_with_quat(
    quat0: &Self,
    quat1: &Self,
  ) -> Self {
    let Quat {
      w: w0,
      x: x0,
      y: y0,
      z: z0,
    } = quat0;
    let Quat {
      w: w1,
      x: x1,
      y: y1,
      z: z1,
    } = quat1;
    Quat {
      w: w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1,
      x: y0 * z1 - z0 * y1 + w0 * x1 + x0 * w1,
      y: z0 * x1 - x0 * z1 + w0 * y1 + y0 * w1,
      z: x0 * y1 - y0 * x1 + w0 * z1 + z0 * w1,
    }
  }
}

// Methods ---------------------------------------------------------------------

impl Quat {
  pub fn matches_closely(
    &self,
    other: &Self,
    tolerance: f64,
  ) -> bool {
    (self.w - other.w).abs() <= tolerance
      && (self.x - other.x).abs() <= tolerance
      && (self.y - other.y).abs() <= tolerance
      && (self.z - other.z).abs() <= tolerance
  }

  pub fn matches_exactly(
    &self,
    other: &Self,
  ) -> bool {
    self.w == other.w
      && self.x == other.x
      && self.y == other.y
      && self.z == other.z
  }

  pub fn multiply_with_quat(
    &mut self,
    multiplier: &Quat,
  ) -> &mut Self {
    let product = Quat::multiply_quat_with_quat(self, multiplier);
    self.w = product.w;
    self.x = product.x;
    self.y = product.y;
    self.z = product.z;
    self
  }
}

// Operator Mul ----------------------------------------------------------------

impl Mul<Quat> for Quat {
  type Output = Quat;

  fn mul(
    self,
    rhs: Quat,
  ) -> Self::Output {
    Quat::multiply_quat_with_quat(&self, &rhs)
  }
}

impl Mul<Quat> for &Quat {
  type Output = Quat;

  fn mul(
    self,
    rhs: Quat,
  ) -> Self::Output {
    Quat::multiply_quat_with_quat(self, &rhs)
  }
}

impl Mul<&Quat> for Quat {
  type Output = Quat;

  fn mul(
    self,
    rhs: &Quat,
  ) -> Self::Output {
    Quat::multiply_quat_with_quat(&self, rhs)
  }
}

impl Mul<&Quat> for &Quat {
  type Output = Quat;

  fn mul(
    self,
    rhs: &Quat,
  ) -> Self::Output {
    Quat::multiply_quat_with_quat(self, rhs)
  }
}

// Operator MulAssign ----------------------------------------------------------

impl MulAssign<Quat> for Quat {
  fn mul_assign(
    &mut self,
    rhs: Quat,
  ) {
    self.multiply_with_quat(&rhs);
  }
}

impl MulAssign<Quat> for &mut Quat {
  fn mul_assign(
    &mut self,
    rhs: Quat,
  ) {
    self.multiply_with_quat(&rhs);
  }
}

impl MulAssign<&Quat> for Quat {
  fn mul_assign(
    &mut self,
    rhs: &Quat,
  ) {
    self.multiply_with_quat(rhs);
  }
}

impl MulAssign<&Quat> for &mut Quat {
  fn mul_assign(
    &mut self,
    rhs: &Quat,
  ) {
    self.multiply_with_quat(rhs);
  }
}

// Trait Default ---------------------------------------------------------------

impl Default for Quat {
  fn default() -> Self {
    Self {
      w: 1.0,
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }
}

// Trait From ------------------------------------------------------------------

impl From<Quat> for AxisAngle {
  fn from(quat: Quat) -> Self {
    let Quat {
      w,
      x,
      y,
      z,
    } = quat;
    let sin_theta_over_2_sq = 1.0 - w * w;
    if sin_theta_over_2_sq <= 0.0 {
      return AxisAngle::default();
    }
    let one_over_sin_theta_over_2 = 1.0 / sin_theta_over_2_sq.sqrt();
    AxisAngle {
      radians: 2.0 * w.acos(),
      x: x * one_over_sin_theta_over_2,
      y: y * one_over_sin_theta_over_2,
      z: z * one_over_sin_theta_over_2,
    }
  }
}

impl From<RotationDegrees> for Quat {
  fn from(rotation_degrees: RotationDegrees) -> Self {
    Quat::from(RotationRadians::from(rotation_degrees))
  }
}

impl From<RotationRadians> for Quat {
  fn from(rotation_radians: RotationRadians) -> Self {
    let axis_angle_x = AxisAngle {
      radians: rotation_radians.x,
      x: 0.0,
      y: 0.0,
      z: 1.0,
    };
    let axis_angle_y = AxisAngle {
      radians: rotation_radians.y,
      x: 0.0,
      y: 1.0,
      z: 0.0,
    };
    let axis_angle_z = AxisAngle {
      radians: rotation_radians.z,
      x: 0.0,
      y: 0.0,
      z: 1.0, // TODO: Is this right?
    };
    let quat_x = Quat::from(axis_angle_x);
    let quat_y = Quat::from(axis_angle_y);
    let quat_z = Quat::from(axis_angle_z);
    quat_z * quat_y * quat_x
  }
}
