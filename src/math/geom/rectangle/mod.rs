// =============================================================================
//! - Rectangle for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-18
//! - Updated: 2023-03-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::point_2dd::Point2DD;

#[cfg(test)]
mod test;

#[derive(Clone, Copy, Debug, Default)]
pub struct Rectangle {
  pub x_max: f64,
  pub x_min: f64,
  pub y_max: f64,
  pub y_min: f64,
}

impl Rectangle {
  pub fn contains(
    &self,
    point2dd: &Point2DD,
  ) -> bool {
    let Point2DD {
      x,
      y,
    } = *point2dd;
    x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
  }
}
