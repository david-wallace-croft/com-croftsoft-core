// =============================================================================
//! - Axis-angle
//!
//! # Metadata
//! - Copyright: &copy; 2008 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-10
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
  pub fn magnitude(&self) -> f64 {
    let x = self.x;
    let y = self.y;
    let z = self.z;
    (x * x + y * y + z * z).sqrt()
  }
}
