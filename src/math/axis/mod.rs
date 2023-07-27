// =============================================================================
//! - Axis-angle (used in 3D graphics)
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2008-05-09
//! - Java updated: 2008-05-09
//! - Rust created: 2022-10-10
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the Java package com.croftsoft.core.math.axis
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

use super::{matrix::structures::Matrix, quat::Quat};

#[cfg(test)]
mod test;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxisAngle {
  pub radians: f64,
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl AxisAngle {
  // ---------------------------------------------------------------------------
  /// Distance from the origin
  // ---------------------------------------------------------------------------
  pub fn magnitude(&self) -> f64 {
    let x = self.x;
    let y = self.y;
    let z = self.z;
    (x * x + y * y + z * z).sqrt()
  }

  // ---------------------------------------------------------------------------
  /// Returns false if any difference magnitude is greater than the tolerance.
  ///
  /// The tolerance should be a positive number.
  // ---------------------------------------------------------------------------
  pub fn matches_closely(
    &self,
    other: &Self,
    tolerance: f64,
  ) -> bool {
    (self.radians - other.radians).abs() <= tolerance
      && (self.x - other.x).abs() <= tolerance
      && (self.y - other.y).abs() <= tolerance
      && (self.z - other.z).abs() <= tolerance
  }

  // ---------------------------------------------------------------------------
  /// Returns false if any field value of other differs from the value of self
  // ---------------------------------------------------------------------------
  pub fn matches_exactly(
    &self,
    other: &Self,
  ) -> bool {
    self.radians == other.radians
      && self.x == other.x
      && self.y == other.y
      && self.z == other.z
  }

  // ---------------------------------------------------------------------------
  /// Divides each entry by the magnitude and then returns a reference to self
  // ---------------------------------------------------------------------------
  pub fn normalize(&mut self) -> &mut Self {
    let magnitude = self.magnitude();
    self.x /= magnitude;
    self.y /= magnitude;
    self.z /= magnitude;
    self
  }
}

// Default ---------------------------------------------------------------------

impl Default for AxisAngle {
  fn default() -> Self {
    Self {
      radians: 0.0,
      x: 1.0,
      y: 0.0,
      z: 0.0,
    }
  }
}

// From ------------------------------------------------------------------------

impl From<AxisAngle> for Matrix<3, 3> {
  fn from(axis_angle: AxisAngle) -> Self {
    let AxisAngle {
      radians,
      x,
      y,
      z,
    } = axis_angle;
    let c = radians.cos();
    let s = radians.sin();
    // Lengyel, "Mathematics for 3D Game Programming & Computer Graphics",
    // Second Edition, p80, equation 3.22.
    Matrix {
      rows: [
        [
          c + (1.0 - c) * x * x,
          (1.0 - c) * x * y - s * z,
          (1.0 - c) * x * z + s * y,
        ],
        [
          (1.0 - c) * x * y + s * z,
          c + (1.0 - c) * y * y,
          (1.0 - c) * y * z - s * x,
        ],
        [
          (1.0 - c) * x * z - s * y,
          (1.0 - c) * y * z + s * x,
          c + (1.0 - c) * z * z,
        ],
      ],
    }
  }
}

impl From<AxisAngle> for Quat {
  fn from(axis_angle: AxisAngle) -> Self {
    let half_radians = axis_angle.radians / 2.0;
    let sin_half_radians = half_radians.sin();
    Quat {
      w: half_radians.cos(),
      x: sin_half_radians * axis_angle.x,
      y: sin_half_radians * axis_angle.y,
      z: sin_half_radians * axis_angle.z,
    }
  }
}
