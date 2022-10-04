// =============================================================================
//! - Unit tests for the Matrix trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-10-04
//! - Since: 2022-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use super::super::structures::*;

#[test]
fn test_default() {
  assert_eq!(
    // A "two by four" matrix of all zeroes
    &Matrix::<2, 4>::default(),
    // A matrix of two rows and four columns
    &Matrix {
      rows: [[0.0; 4]; 2]
    }
  );
}

#[test]
fn test_from_rotation_degrees_to_matrix() {
  assert_eq!(
    Matrix::from(RotationDegrees {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }),
    Matrix::identity()
  );
  assert!(Matrix::from(RotationDegrees {
    x: 90.0,
    y: 0.0,
    z: 0.0,
  })
  .matches_closely(
    &Matrix {
      rows: [
        [1.0, 0.0, 0.0],
        [0.0, 0.0, -1.0],
        [0.0, 1.0, 0.0],
      ],
    },
    0.001
  ));
  assert!(Matrix::from(RotationDegrees {
    x: 0.0,
    y: 90.0,
    z: 0.0,
  })
  .matches_closely(
    &Matrix {
      rows: [
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0],
        [-1.0, 0.0, 0.0],
      ],
    },
    0.001
  ));
  assert!(Matrix::from(RotationDegrees {
    x: 0.0,
    y: 0.0,
    z: 90.0,
  })
  .matches_closely(
    &Matrix {
      rows: [
        [0.0, -1.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
      ],
    },
    0.001
  ));
  let rotation_matrix: Matrix<3, 3> = RotationDegrees {
    x: 0.0,
    y: 0.0,
    z: 0.0,
  }
  .into();
  assert_eq!(rotation_matrix, Matrix::identity());
}

#[test]
fn test_from_rotation_degrees_to_rotation_radians() {
  assert_eq!(
    RotationRadians::from(RotationDegrees {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }),
    RotationRadians {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  );
  assert_eq!(
    RotationRadians::from(RotationDegrees {
      x: 90.0,
      y: 0.0,
      z: 0.0,
    }),
    RotationRadians {
      x: core::f64::consts::FRAC_PI_2,
      y: 0.0,
      z: 0.0,
    }
  );
  let rotation_radians: RotationRadians = RotationDegrees {
    x: 0.0,
    y: 0.0,
    z: 0.0,
  }
  .into();
  assert_eq!(
    rotation_radians,
    RotationRadians {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  );
}

#[test]
fn test_from_rotation_radians_to_matrix() {
  assert_eq!(
    Matrix::from(RotationRadians {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }),
    Matrix::identity()
  );
  let rotation_matrix: Matrix<3, 3> = RotationRadians {
    x: 0.0,
    y: 0.0,
    z: 0.0,
  }
  .into();
  assert_eq!(rotation_matrix, Matrix::identity());
}

#[test]
fn test_from_rotation_radians_to_rotation_degrees() {
  assert_eq!(
    RotationDegrees::from(RotationRadians {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }),
    RotationDegrees {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  );
  assert_eq!(
    RotationDegrees::from(RotationRadians {
      x: core::f64::consts::FRAC_PI_2,
      y: 0.0,
      z: 0.0,
    }),
    RotationDegrees {
      x: 90.0,
      y: 0.0,
      z: 0.0,
    }
  );
  let rotation_degrees: RotationDegrees = RotationRadians {
    x: 0.0,
    y: 0.0,
    z: 0.0,
  }
  .into();
  assert_eq!(
    rotation_degrees,
    RotationDegrees {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  );
}
