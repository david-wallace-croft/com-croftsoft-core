// =============================================================================
//! - Quaternion
//!
//! # Metadata
//! - Copyright: &copy; 2008 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-10
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

#[cfg(test)]
mod test;

use super::axis::AxisAngle;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
  pub w: f64,
  pub x: f64,
  pub y: f64,
  pub z: f64,
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
