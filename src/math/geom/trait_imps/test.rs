// =============================================================================
//! - Unit tests for the geometry module trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-10-24
//! - Since: 2022-10-23
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use super::super::structures::*;
#[cfg(test)]
use crate::math::geom::traits::PointXY;
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
