// =============================================================================
//! - Associated functions for the geometry module
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-09
//! - Rust since: 2022-11-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::structures::Point2DD;

impl Point2DD {
  pub fn new(
    x: f64,
    y: f64,
  ) -> Point2DD {
    Point2DD {
      x,
      y,
    }
  }
}
