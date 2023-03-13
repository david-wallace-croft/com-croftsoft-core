// =============================================================================
//! - Trait implementations for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2003-03-20
//! - Java updated: 2003-04-13
//! - Rust created: 2022-10-23
//! - Rust updated: 2023-03-12
//!
//! # History
//! - Adapted from the classes in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.math.geom.CircleAccessor
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

impl CircleAccessor for Circle {
  fn get_center_x(&self) -> f64 {
    self.center_x
  }

  fn get_center_y(&self) -> f64 {
    self.center_y
  }

  fn get_radius(&self) -> f64 {
    self.radius
  }
}

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
