// =============================================================================
//! - Point2DD for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2003-03-20
//! - Java updated: 2003-04-13
//! - Rust created: 2023-03-18
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the class in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.math.geom.Point2DD
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

#[cfg(test)]
mod test;

use super::point_xy::PointXY;
use core::fmt::Display;
use core::hash::Hash;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point2DD {
  pub x: f64,
  pub y: f64,
}

impl Point2DD {
  pub fn distance_to(
    &self,
    other: &Point2DD,
  ) -> f64 {
    ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
  }

  pub fn is_near(
    &self,
    other: &Point2DD,
    tolerance: f64,
  ) -> bool {
    self.distance_to(other) <= tolerance
  }

  pub fn new(
    x: f64,
    y: f64,
  ) -> Point2DD {
    Point2DD {
      x,
      y,
    }
  }

  pub fn set_x(
    &mut self,
    x: f64,
  ) {
    self.x = x;
  }

  pub fn set_xy(
    &mut self,
    x: f64,
    y: f64,
  ) {
    self.x = x;
    self.y = y;
  }

  pub fn set_xy_point(
    &mut self,
    point_2dd: &Self,
  ) {
    self.x = point_2dd.x;
    self.y = point_2dd.y;
  }

  pub fn set_y(
    &mut self,
    y: f64,
  ) {
    self.y = y;
  }
}

impl Display for Point2DD {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
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
