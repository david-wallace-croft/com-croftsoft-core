// =============================================================================
//! - Unit tests for Rectangle
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

#[cfg(test)]
use crate::math::geom::point_2dd::Point2DD;
#[cfg(test)]
use crate::math::geom::rectangle::Rectangle;

#[cfg(test)]
const TEST_DISTANCE_FROM_ORIGIN: f64 = 10.0;

#[cfg(test)]
const TEST_RECTANGLE: Rectangle = Rectangle {
  x_max: TEST_DISTANCE_FROM_ORIGIN,
  x_min: -TEST_DISTANCE_FROM_ORIGIN,
  y_max: TEST_DISTANCE_FROM_ORIGIN,
  y_min: -TEST_DISTANCE_FROM_ORIGIN,
};

#[test]
fn test_contains() {
  assert!(TEST_RECTANGLE.contains(&Point2DD::default()));
  assert!(TEST_RECTANGLE.contains(&Point2DD {
    x: TEST_DISTANCE_FROM_ORIGIN,
    y: TEST_DISTANCE_FROM_ORIGIN,
  }));
  assert!(!TEST_RECTANGLE.contains(&Point2DD {
    x: TEST_DISTANCE_FROM_ORIGIN + 1.0,
    y: 0.0,
  }));
}
