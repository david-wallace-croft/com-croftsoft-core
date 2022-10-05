// =============================================================================
//! - Unit tests for the Matrix functions and methods
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-10-05
//! - Since: 2022-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use super::super::structures::*;

#[test]
fn test_add_matrix_with_matrix() {
  assert_eq!(
    Matrix::add_matrix_with_matrix(
      &Matrix::<1, 1>::new(1.0),
      &Matrix::new(2.0)
    ),
    Matrix::new(3.0)
  );
}

#[test]
fn test_add_matrix_with_scalar() {
  assert_eq!(
    Matrix::add_matrix_with_scalar(&Matrix::<1, 1>::new(1.0), 2.0),
    Matrix::new(3.0)
  );
}

#[test]
fn test_divide_matrix_by_matrix_entrywise() {
  let dividend_matrix_1x4 = Matrix {
    rows: [[
      0.0, 3.0, 6.0, 9.0,
    ]],
  };
  let divisor_matrix_1x4 = Matrix {
    rows: [[
      4.0, 3.0, 2.0, 1.0,
    ]],
  };
  let expected_quotient_matrix_1x4 = Matrix {
    rows: [[
      0.0, 1.0, 3.0, 9.0,
    ]],
  };
  assert_eq!(
    Matrix::divide_matrix_by_matrix_entrywise(
      &dividend_matrix_1x4,
      &divisor_matrix_1x4
    ),
    expected_quotient_matrix_1x4
  );
}

#[test]
fn test_divide_matrix_by_scalar() {
  assert_eq!(
    Matrix::divide_matrix_by_scalar(&Matrix::<1, 1>::new(6.0), 3.0),
    Matrix::<1, 1>::new(2.0)
  );
}

#[test]
fn test_identity() {
  assert_eq!(
    &Matrix::identity(),
    &Matrix {
      rows: [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0]
      ]
    }
  );
  assert_eq!(Matrix::<3, 3>::identity().sum_entries(), 3.0);
  assert_eq!(
    Matrix {
      rows: [
        [1.0, 2.0],
        [3.0, 4.0]
      ]
    }
    .multiply_with_matrix(&Matrix::identity()),
    &Matrix {
      rows: [
        [1.0, 2.0],
        [3.0, 4.0]
      ]
    }
  );
}

#[test]
fn test_multiply_matrix_with_matrix() {
  assert_eq!(
    Matrix::multiply_matrix_with_matrix(
      &Matrix::<2, 4>::new(2.0),
      &Matrix::<4, 3>::new(3.0)
    ),
    Matrix::<2, 3>::new(24.0)
  );
}

#[test]
fn test_multiply_matrix_with_matrix_entrywise() {
  let original_matrix_1x4 = Matrix {
    rows: [[
      0.0, 1.0, 2.0, 3.0,
    ]],
  };
  let weighting_matrix_1x4 = Matrix {
    rows: [[
      4.0, 3.0, 2.0, 1.0,
    ]],
  };
  let expected_hadamard_product_1x4 = Matrix {
    rows: [[
      0.0, 3.0, 4.0, 3.0,
    ]],
  };
  assert_eq!(
    Matrix::multiply_matrix_with_matrix_entrywise(
      &original_matrix_1x4,
      &weighting_matrix_1x4
    ),
    expected_hadamard_product_1x4
  );
}

#[test]
fn test_multiply_matrix_with_scalar() {
  assert_eq!(
    Matrix::multiply_matrix_with_scalar(&Matrix::<1, 1>::new(2.0), 3.0),
    Matrix::new(6.0)
  );
}

#[test]
fn test_negate_matrix() {
  let matrix = Matrix::<1, 1>::new(1.0);
  assert_eq!(Matrix::negate_matrix(&matrix), Matrix::new(-1.0));
}

#[test]
fn test_new() {
  assert_eq!(
    // A 2x4 matrix from a new() constructor
    &Matrix::<2, 4>::new(0.0),
    // The same with the dimensions inferred
    &Matrix::default()
  );
}

#[test]
fn test_subtract_matrix_from_matrix() {
  assert_eq!(
    Matrix::subtract_matrix_from_matrix(
      &Matrix::<1, 1>::new(3.0),
      &Matrix::new(2.0)
    ),
    Matrix::new(1.0)
  );
}

#[test]
fn test_subtract_matrix_from_scalar() {
  assert_eq!(
    Matrix::subtract_matrix_from_scalar(3.0, &Matrix::<1, 1>::new(2.0)),
    Matrix::new(1.0)
  );
}

#[test]
fn test_subtract_scalar_from_matrix() {
  assert_eq!(
    Matrix::subtract_scalar_from_matrix(&Matrix::<1, 1>::new(3.0), 2.0),
    Matrix::new(1.0)
  );
}

#[test]
fn test_to_rotation_matrix_x_from_degrees() {
  assert_eq!(
    Matrix::to_rotation_matrix_x_from_degrees(Degrees(0.0)),
    Matrix::identity()
  )
}

#[test]
fn test_to_rotation_matrix_x_from_radians() {
  assert_eq!(
    Matrix::to_rotation_matrix_x_from_radians(Radians(0.0)),
    Matrix::identity()
  )
}

#[test]
fn test_to_rotation_matrix_y_from_degrees() {
  assert_eq!(
    Matrix::to_rotation_matrix_y_from_degrees(Degrees(0.0)),
    Matrix::identity()
  )
}

#[test]
fn test_to_rotation_matrix_y_from_radians() {
  assert_eq!(
    Matrix::to_rotation_matrix_y_from_radians(Radians(0.0)),
    Matrix::identity()
  )
}

#[test]
fn test_to_rotation_matrix_z_from_degrees() {
  assert_eq!(
    Matrix::to_rotation_matrix_z_from_degrees(Degrees(0.0)),
    Matrix::identity()
  )
}

#[test]
fn test_to_rotation_matrix_z_from_radians() {
  assert_eq!(
    Matrix::to_rotation_matrix_z_from_radians(Radians(0.0)),
    Matrix::identity()
  )
}
