// =============================================================================
//! - Circle for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 2003-03-20
//! - Java updated: 2003-04-13
//! - Rust created: 2023-03-18
//! - Rust updated: 2023-04-28
//!
//! # History
//! - Adapted from the class in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.math.geom.Circle
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::point_2dd::Point2DD;

#[derive(Clone, Copy, Debug, Default)]
pub struct Circle {
  pub center_x: f64,
  pub center_y: f64,
  pub radius: f64,
}

pub trait CircleAccessor {
  fn get_center_point_2dd(&self) -> Point2DD;

  fn get_center_x(&self) -> f64;

  fn get_center_y(&self) -> f64;

  fn get_radius(&self) -> f64;

  fn intersects_circle(
    &self,
    other: &dyn CircleAccessor,
  ) -> bool;

  // TODO
  // fn intersects_shape(&self, other_shape: dyn Shape) -> bool;
}

impl Circle {
  pub fn contains(
    &self,
    x: f64,
    y: f64,
  ) -> bool {
    let distance =
      ((self.center_x - x).powi(2) + (self.center_y - y).powi(2)).sqrt();
    distance <= self.radius
  }

  pub fn new(other: &dyn CircleAccessor) -> Self {
    Self {
      center_x: other.get_center_x(),
      center_y: other.get_center_y(),
      radius: other.get_radius(),
    }
  }

  pub fn set_center(
    &mut self,
    center_x: f64,
    center_y: f64,
  ) {
    self.center_x = center_x;
    self.center_y = center_y;
  }

  pub fn set_center_from_circle(
    &mut self,
    circle: &Circle,
  ) {
    self.center_x = circle.center_x;
    self.center_y = circle.center_y;
  }

  pub fn set_center_from_point_2dd(
    &mut self,
    point_2dd: &Point2DD,
  ) {
    self.center_x = point_2dd.x;
    self.center_y = point_2dd.y;
  }
}

impl CircleAccessor for Circle {
  fn get_center_point_2dd(&self) -> Point2DD {
    Point2DD {
      x: self.center_x,
      y: self.center_y,
    }
  }

  fn get_center_x(&self) -> f64 {
    self.center_x
  }

  fn get_center_y(&self) -> f64 {
    self.center_y
  }

  fn get_radius(&self) -> f64 {
    self.radius
  }

  fn intersects_circle(
    &self,
    other: &dyn CircleAccessor,
  ) -> bool {
    let distance = ((self.center_x - other.get_center_x()).powi(2)
      + (self.center_y - other.get_center_y()).powi(2))
    .sqrt();
    distance <= self.radius + other.get_radius()
  }
}
