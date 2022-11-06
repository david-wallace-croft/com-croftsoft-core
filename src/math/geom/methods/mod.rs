// =============================================================================
//! - Methods for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-06
//! - Rust since: 2022-11-06
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

mod test;

use super::structures::Point2DD;

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
}
