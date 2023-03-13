// =============================================================================
//! - Traits for the geometry module
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
//! - Adapted from the interfaces in the Java-based [`CroftSoft Core Library`]
//!   - com.croftsoft.core.math.geom.CircleAccessor
//!   - com.croftsoft.core.math.geom.PointXY
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use core::hash::Hash;

pub trait CircleAccessor {
  fn get_center_x(&self) -> f64;
  fn get_center_y(&self) -> f64;
  fn get_radius(&self) -> f64;
  // TODO
  // fn intersects_circle(&self, other_circle: dyn CircleAccessor) -> bool;
  // fn intersects_shape(&self, other_shape: dyn Shape) -> bool;
}

pub trait PointXY: Clone + Copy + Eq + Hash {
  /// The angle, in radians, from this point to the other point
  fn angle_to<P: PointXY>(
    &self,
    other_pointxy: &P,
  ) -> f64;

  fn distance(
    &self,
    other_x: f64,
    other_y: f64,
  ) -> f64;

  fn distance_xy<P: PointXY>(
    &self,
    other_pointxy: &P,
  ) -> f64;

  fn get_x(&self) -> f64;

  fn get_y(&self) -> f64;
}
