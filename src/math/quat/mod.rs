// =============================================================================
//! - Quaternion
//!
//! # Metadata
//! - Copyright: &copy; 2008 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-18
//! - Rust since: 2022-10-10
//! - Java version: 2008-05-09
//! - Java since: 2008-05-02
//!
//! # History
//! - Adapted from the Java package com.croftsoft.core.math.quat
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::axis::AxisAngle;
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
