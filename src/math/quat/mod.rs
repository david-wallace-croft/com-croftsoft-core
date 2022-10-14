// =============================================================================
//! - Quaternion
//!
//! # Metadata
//! - Copyright: &copy; 2008 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-14
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

#[cfg(test)]
mod test;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
  pub w: f64,
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

// Default ---------------------------------------------------------------------

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

// From ------------------------------------------------------------------------

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
