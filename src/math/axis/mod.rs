// =============================================================================
//! - Axis-angle (used in 3D graphics)
//!
//! # Metadata
//! - Copyright: &copy; 2008 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-13
//! - Rust since: 2022-10-10
//! - Java version: 2008-05-09
//! - Java since: 2008-05-09
//!
//! # History
//! - Adapted from the Java package com.croftsoft.core.math.axis
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

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
