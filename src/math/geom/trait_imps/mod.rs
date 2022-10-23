// =============================================================================
//! - Trait implementations for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 1998 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-23
//! - Rust since: 2022-10-23
//! - Java version: 2003-04-13
//! - Java since: 2003-03-20
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.math.geom.Point2DD
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
mod test;

use super::structures::*;
use super::traits::*;

impl PointXY for Point2DD {
  fn angle_to(
    &self,
    other_pointxy: &dyn PointXY,
  ) -> f64 {
    (other_pointxy.get_y() - self.y).atan2(other_pointxy.get_x() - self.x)
  }

  fn distance(
    &self,
    other_x: f64,
    other_y: f64,
  ) -> f64 {
    ((self.x - other_x).powi(2) + (self.y - other_y).powi(2)).sqrt()
  }

  fn distance_xy(
    &self,
    other_pointxy: &dyn PointXY,
  ) -> f64 {
    self.distance(other_pointxy.get_x(), other_pointxy.get_y())
  }

  fn get_x(&self) -> f64 {
    self.x
  }

  fn get_y(&self) -> f64 {
    self.y
  }
}
