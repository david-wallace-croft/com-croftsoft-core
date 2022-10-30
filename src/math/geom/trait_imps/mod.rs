// =============================================================================
//! - Trait implementations for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 1998 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-30
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
use core::hash::Hash;

impl Eq for Point2DD {}

impl Hash for Point2DD {
  fn hash<H: std::hash::Hasher>(
    &self,
    state: &mut H,
  ) {
    self.x.to_bits().hash(state);
    self.y.to_bits().hash(state);
  }
}

impl PartialEq for Point2DD {
  fn eq(
    &self,
    other: &Self,
  ) -> bool {
    self.x.to_bits() == other.x.to_bits()
      && self.y.to_bits() == other.y.to_bits()
  }
}

impl PointXY for Point2DD {
  fn angle_to<P: PointXY>(
    &self,
    other_pointxy: &P,
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

  fn distance_xy<P: PointXY>(
    &self,
    other_pointxy: &P,
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
