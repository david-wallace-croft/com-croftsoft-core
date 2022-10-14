// =============================================================================
//! - Unit tests for the Matrix trait implementations
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-10-14
//! - Since: 2022-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use super::super::structures::*;
#[cfg(test)]
use core::f64::consts::FRAC_PI_2;

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
fn test_from_degrees_to_radians() {
  assert_eq!(Radians::from(Degrees(0.0)), Radians(0.0));
  assert_eq!(Radians(0.0), Degrees(0.0).into());
}

#[test]
fn test_from_radians_to_degrees() {
  assert_eq!(Degrees::from(Radians(0.0)), Degrees(0.0));
  assert_eq!(Degrees(0.0), Radians(0.0).into());
}

#[test]
fn test_from_matrix_to_rotation_degrees() {
  let rotation_matrix = Matrix {
    rows: [
      [
        1.0, 0.0, 0.0,
      ],
      [
        0.0, 1.0, 0.0,
      ],
      [
        0.0, 0.0, 1.0,
      ],
    ],
  };
  assert_eq!(
    RotationDegrees::from(rotation_matrix),
    RotationDegrees {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  );
  let rotation_matrix = Matrix {
    rows: [
      [
        1.0, 0.0, 0.0,
      ],
      [
        0.0, 0.0, -1.0,
      ],
      [
        0.0, 1.0, 0.0,
      ],
    ],
  };
  assert_eq!(
    RotationDegrees::from(rotation_matrix),
    RotationDegrees {
      x: 90.0,
      y: 0.0,
      z: 0.0,
    }
  );
}

#[test]
fn test_from_matrix_to_rotation_radians() {
  let rotation_matrix = Matrix {
    rows: [
      [
        0.0, 0.0, 1.0,
      ],
      [
        0.0, 1.0, 0.0,
      ],
      [
        -1.0, 0.0, 0.0,
      ],
    ],
  };
  assert_eq!(
    RotationRadians::from(rotation_matrix),
    RotationRadians {
      x: 0.0,
      y: FRAC_PI_2,
      z: 0.0,
    }
  );
  let rotation_matrix = Matrix {
    rows: [
      [
        0.0, -1.0, 0.0,
      ],
      [
        1.0, 0.0, 0.0,
      ],
      [
        0.0, 0.0, 1.0,
      ],
    ],
  };
  assert_eq!(
    RotationRadians::from(rotation_matrix),
    RotationRadians {
      x: 0.0,
      y: 0.0,
      z: FRAC_PI_2,
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
  assert_eq!(
    Matrix::identity(),
    RotationDegrees {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
    .into()
  );
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
      x: FRAC_PI_2,
      y: 0.0,
      z: 0.0,
    }
  );
  assert_eq!(
    RotationRadians {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    RotationDegrees {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
    .into(),
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
  assert_eq!(
    Matrix::identity(),
    RotationRadians {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
    .into()
  );
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
      x: FRAC_PI_2,
      y: 0.0,
      z: 0.0,
    }),
    RotationDegrees {
      x: 90.0,
      y: 0.0,
      z: 0.0,
    }
  );
  assert_eq!(
    RotationDegrees {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    RotationRadians {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
    .into()
  );
}
