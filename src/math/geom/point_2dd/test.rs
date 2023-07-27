// =============================================================================
//! - Unit tests for Point2DD
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-18
//! - Updated: 2023-07-26
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

#[cfg(test)]
use crate::math::geom::point_2dd::Point2DD;
#[cfg(test)]
use crate::math::geom::point_xy::PointXY;
#[cfg(test)]
use core::f64::consts::FRAC_PI_2;

#[test]
fn test_angle_to() {
  assert_eq!(
    Point2DD::default().angle_to(&Point2DD {
      x: 0.0,
      y: 1.0
    }),
    FRAC_PI_2
  );
}

#[test]
fn test_distance() {
  assert!(Point2DD::default().distance(1.0, 1.0) - 1.414 < 0.001);
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
fn test_distance_xy() {
  assert!(
    Point2DD::default().distance_xy(&Point2DD {
      x: 1.0,
      y: 1.0
    }) - 1.414
      < 0.001
  );
}

#[test]
fn test_get_x() {
  assert_eq!(Point2DD::default().get_x(), 0.0);
  assert_eq!(
    Point2DD {
      x: 1.0,
      y: 2.0
    }
    .get_x(),
    1.0
  );
}

#[test]
fn test_get_y() {
  assert_eq!(Point2DD::default().get_y(), 0.0);
  assert_eq!(
    Point2DD {
      x: 1.0,
      y: 2.0
    }
    .get_y(),
    2.0
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
