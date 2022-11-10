// =============================================================================
//! - Unit tests for the geometry module methods
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-11-10
//! - Rust since: 2022-11-06
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use crate::math::geom::structures::{Point2DD, Rectangle};

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

#[test]
fn test_distance_to() {
  assert!(
    Point2DD::default().distance_to(&Point2DD {
      x: 0.707,
      y: 0.707
    }) - 1.0
      <= 0.001
  );
}

#[test]
fn test_is_near() {
  assert!(Point2DD::default().is_near(
    &Point2DD {
      x: 1.0,
      y: 0.0
    },
    1.0
  ));
  assert!(!Point2DD::default().is_near(
    &Point2DD {
      x: 2.0,
      y: 0.0
    },
    1.0
  ));
}
