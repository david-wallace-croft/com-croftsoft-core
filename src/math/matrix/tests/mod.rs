// =============================================================================
//! - Unit tests for the Matrix functions and methods
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-09-25
//! - Since: 2022-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use super::*;

#[test]
fn test_add() {
  // augend and f64
  assert_eq!(Matrix::<1, 1>::new(1.0) + 2.0, Matrix::new(3.0));
  // &augend and f64
  assert_eq!(&Matrix::<1, 1>::new(1.0) + 2.0, Matrix::new(3.0));
  // f64 and addend
  assert_eq!(1.0 + Matrix::<1, 1>::new(2.0), Matrix::new(3.0));
  // f64 and &addend
  assert_eq!(1.0 + &Matrix::<1, 1>::new(2.0), Matrix::new(3.0));
  // augend and addend
  assert_eq!(
    Matrix::<1, 1>::new(1.0) + Matrix::new(2.0),
    Matrix::new(3.0)
  );
  // augend and &addend
  assert_eq!(
    Matrix::<1, 1>::new(1.0) + &Matrix::new(2.0),
    Matrix::new(3.0)
  );
  // &augend and addend
  assert_eq!(
    &Matrix::<1, 1>::new(1.0) + Matrix::new(2.0),
    Matrix::new(3.0)
  );
  // &augend and &addend
  assert_eq!(
    &Matrix::<1, 1>::new(1.0) + &Matrix::new(2.0),
    Matrix::new(3.0)
  );
}

#[test]
fn test_add_assign() {
  let matrix_3 = Matrix::<1, 1>::new(3.0);
  // augend and f64
  let mut matrix = Matrix::<1, 1>::new(1.0);
  matrix += 2.0;
  assert_eq!(matrix, matrix_3);
  // &augend and f64
  let mut matrix = &mut Matrix::<1, 1>::new(1.0);
  matrix += 2.0;
  assert_eq!(matrix, &matrix_3);
  // augend and addend
  let mut matrix = Matrix::<1, 1>::new(1.0);
  matrix += Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, matrix_3);
  // augend and &addend
  let mut matrix = Matrix::<1, 1>::new(1.0);
  matrix += &Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, matrix_3);
  // &augend and addend
  let mut matrix = &mut Matrix::<1, 1>::new(1.0);
  matrix += Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, &matrix_3);
  // &augend and &addend
  let mut matrix = &mut Matrix::<1, 1>::new(1.0);
  matrix += &Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, &matrix_3);
}

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
fn test_add_with_matrix() {
  assert_eq!(
    Matrix::<2, 4>::new(1.0).add_with_matrix(&Matrix::new(1.0)),
    &Matrix::new(2.0)
  );
}

#[test]
fn test_add_with_scalar() {
  assert_eq!(
    // A 2x4 matrix of all ones
    &Matrix::<2, 4>::new(1.0),
    // The same by adding 1 to the default
    Matrix::default().add_with_scalar(1.0)
  );
}

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
fn test_get_entry() {
  // first row, last column
  let indices = Indices {
    row: 0,
    column: 3,
  };
  assert_eq!(
    Matrix::<2, 4>::default().set_entry(indices, 1.0).get_entry(indices),
    1.0
  );
}

#[test]
fn test_get_row() {
  // first row, last column
  let indices = Indices {
    row: 0,
    column: 3,
  };
  assert_eq!(
    Matrix::<2, 4>::default().set_entry(indices, 1.0).get_row(0),
    &[0.0, 0.0, 0.0, 1.0]
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
  assert_eq!(Matrix::<3, 3>::identity().sum(), 3.0);
  assert_eq!(
    Matrix {
      rows: [
        [1.0, 2.0],
        [3.0, 4.0]
      ]
    }
    .multiply_with_square_matrix(&Matrix::identity()),
    &Matrix {
      rows: [
        [1.0, 2.0],
        [3.0, 4.0]
      ]
    }
  );
}

#[test]
fn test_matches_closely() {
  assert!(
    Matrix::<2, 4>::new(1.0 / 3.0).matches_closely(&Matrix::new(0.33), 0.01)
  );
  assert!(Matrix::<2, 4>::new(0.0).matches_exactly(&Matrix::default()));
}

#[test]
fn test_mul() {
  let matrix_2 = Matrix::<1, 1>::new(2.0);
  // multiplicand and f64
  assert_eq!(Matrix::<1, 1>::new(1.0) * 2.0, matrix_2);
  // &multiplicand and f64
  assert_eq!(&Matrix::<1, 1>::new(1.0) * 2.0, matrix_2);
  // f64 and multiplier
  assert_eq!(2.0 * Matrix::<1, 1>::new(1.0), matrix_2);
  // f64 and &multiplier
  assert_eq!(2.0 * &Matrix::<1, 1>::new(1.0), matrix_2);
  // multiplicand and multiplier
  assert_eq!(
    Matrix::<1, 2>::new(1.0) * Matrix::<2, 3>::new(2.0),
    Matrix::<1, 3>::new(4.0)
  );
  // multiplicand and &multiplier
  assert_eq!(
    Matrix::<1, 2>::new(1.0) * &Matrix::<2, 3>::new(2.0),
    Matrix::<1, 3>::new(4.0)
  );
  // &multiplicand and multiplier
  assert_eq!(
    &Matrix::<1, 2>::new(1.0) * Matrix::<2, 3>::new(2.0),
    Matrix::<1, 3>::new(4.0)
  );
  // &multiplicand and &multiplier
  assert_eq!(
    &Matrix::<1, 2>::new(1.0) * &Matrix::<2, 3>::new(2.0),
    Matrix::<1, 3>::new(4.0)
  );
}

#[test]
fn test_mul_assign() {
  // muliplicand and f64
  let mut multiplicand = Matrix::<1, 2>::new(1.0);
  multiplicand *= 2.0;
  assert_eq!(multiplicand, Matrix::<1, 2>::new(2.0));
  // &muliplicand and f64
  let mut multiplicand = &mut Matrix::<1, 2>::new(1.0);
  multiplicand *= 2.0;
  assert_eq!(multiplicand, &Matrix::<1, 2>::new(2.0));
  // muliplicand and multiplier
  let mut multiplicand = Matrix::<1, 2>::new(2.0);
  let multiplier = Matrix::<2, 2>::new(3.0);
  multiplicand *= multiplier;
  assert_eq!(multiplicand, Matrix::<1, 2>::new(12.0));
  // muliplicand and &multiplier
  let mut multiplicand = Matrix::<1, 2>::new(2.0);
  let multiplier = &Matrix::<2, 2>::new(3.0);
  multiplicand *= multiplier;
  assert_eq!(multiplicand, Matrix::<1, 2>::new(12.0));
  // &muliplicand and multiplier
  let mut multiplicand = &mut Matrix::<1, 2>::new(2.0);
  let multiplier = Matrix::<2, 2>::new(3.0);
  multiplicand *= multiplier;
  assert_eq!(multiplicand, &Matrix::<1, 2>::new(12.0));
  // &muliplicand and &multiplier
  let mut multiplicand = &mut Matrix::<1, 2>::new(2.0);
  let multiplier = &Matrix::<2, 2>::new(3.0);
  multiplicand *= multiplier;
  assert_eq!(multiplicand, &Matrix::<1, 2>::new(12.0));
}

#[test]
fn test_multiply_with_corresponding_entries() {
  let mut matrix_1x4 = Matrix {
    rows: [[
      0.0, 1.0, 2.0, 3.0,
    ]],
  };
  let weighting_matrix = Matrix {
    rows: [[
      4.0, 3.0, 2.0, 1.0,
    ]],
  };
  let expected_hadamard_product = Matrix {
    rows: [[
      0.0, 3.0, 4.0, 3.0,
    ]],
  };
  assert_eq!(
    matrix_1x4.multiply_with_corresponding_entries(&weighting_matrix),
    &expected_hadamard_product
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
fn test_multiply_matrix_with_scalar() {
  assert_eq!(
    Matrix::multiply_matrix_with_scalar(&Matrix::<1, 1>::new(2.0), 3.0),
    Matrix::new(6.0)
  );
}

#[test]
fn test_multiply_with_square_matrix() {
  assert_eq!(
    Matrix::<2, 4>::new(2.0)
      .multiply_with_square_matrix(&Matrix::<4, 4>::new(3.0)),
    &Matrix::<2, 4>::new(24.0)
  );
  let mut matrix_multiplicand = Matrix {
    rows: [
      [
        1.0, 0.0, 1.0,
      ],
      [
        2.0, 1.0, 1.0,
      ],
      [
        0.0, 1.0, 1.0,
      ],
      [
        1.0, 1.0, 2.0,
      ],
    ],
  };
  let matrix_multiplier = Matrix {
    rows: [
      [
        1.0, 2.0, 1.0,
      ],
      [
        2.0, 3.0, 1.0,
      ],
      [
        4.0, 2.0, 2.0,
      ],
    ],
  };
  let expected_matrix_product = Matrix {
    rows: [
      [
        5.0, 4.0, 3.0,
      ],
      [
        8.0, 9.0, 5.0,
      ],
      [
        6.0, 5.0, 3.0,
      ],
      [
        11.0, 9.0, 6.0,
      ],
    ],
  };
  assert_eq!(
    matrix_multiplicand.multiply_with_square_matrix(&matrix_multiplier),
    &expected_matrix_product
  );
}

#[test]
fn test_multiply_scalar() {
  assert_eq!(
    Matrix::<2, 4>::new(3.0).multiply_with_scalar(5.0),
    &Matrix::new(15.0)
  );
}

#[test]
fn test_neg() {
  // matrix
  assert_eq!(-Matrix::<1, 1>::new(1.0), Matrix::new(-1.0));
  // &matrix
  assert_eq!(-&Matrix::<1, 1>::new(1.0), Matrix::new(-1.0));
}

#[test]
fn test_negate() {
  let mut matrix = Matrix::<1, 1>::new(1.0);
  assert_eq!(matrix.negate(), &Matrix::new(-1.0));
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
fn test_sub() {
  let matrix_1 = Matrix::<1, 1>::new(1.0);
  // minuend and f64
  assert_eq!(Matrix::<1, 1>::new(3.0) - 2.0, matrix_1);
  // &minuend and f64
  assert_eq!(&Matrix::<1, 1>::new(3.0) - 2.0, matrix_1);
  // f64 and subtrahend
  assert_eq!(3.0 - Matrix::<1, 1>::new(2.0), matrix_1);
  // f64 and &subtrahend
  assert_eq!(3.0 - &Matrix::<1, 1>::new(2.0), matrix_1);
  // minuend and subtrahend
  assert_eq!(Matrix::<1, 1>::new(3.0) - Matrix::new(2.0), matrix_1);
  // minuend and &subtrahend
  assert_eq!(Matrix::<1, 1>::new(3.0) - &Matrix::new(2.0), matrix_1);
  // &minuend and subtrahend
  assert_eq!(&Matrix::<1, 1>::new(3.0) - Matrix::new(2.0), matrix_1);
  // &minuend and &subtahend
  assert_eq!(&Matrix::<1, 1>::new(3.0) - &Matrix::new(2.0), matrix_1);
}

#[test]
fn test_sub_assign() {
  let matrix_1 = Matrix::<1, 1>::new(1.0);
  // minuend and f64
  let mut matrix = Matrix::<1, 1>::new(3.0);
  matrix -= 2.0;
  assert_eq!(matrix, matrix_1);
  // &minuend and f64
  let mut matrix = &mut Matrix::<1, 1>::new(3.0);
  matrix -= 2.0;
  assert_eq!(matrix, &matrix_1);
  // minuend and subtrahend
  let mut matrix = Matrix::<1, 1>::new(3.0);
  matrix -= Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, matrix_1);
  // minuend and &subtrahend
  let mut matrix = Matrix::<1, 1>::new(3.0);
  matrix -= &Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, matrix_1);
  // &minuend and subtrahend
  let mut matrix = &mut Matrix::<1, 1>::new(3.0);
  matrix -= Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, &matrix_1);
  // &minuend and &subtrahend
  let mut matrix = &mut Matrix::<1, 1>::new(3.0);
  matrix -= &Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, &matrix_1);
}

#[test]
fn test_submatrix() {
  assert_eq!(
    Matrix {
      rows: [
        [0.0, 1.0, 2.0],
        [3.0, 4.0, 5.0]
      ]
    }
    .submatrix::<1, 2>(Indices {
      row: 1,
      column: 0
    }),
    Matrix {
      rows: [[3.0, 4.0]]
    }
  );
}

#[test]
fn test_subtract_matrix() {
  assert_eq!(
    Matrix::<2, 4>::new(3.0).subtract_matrix(&Matrix::new(2.0)),
    &Matrix::new(1.0)
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
fn test_subtract_scalar_from_self() {
  assert_eq!(
    // A 2x4 matrix of negative ones
    &Matrix::<2, 4>::new(-1.0),
    // The same by subtracting one
    Matrix::default().subtract_scalar_from_self(1.0)
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
fn test_sum() {
  assert_eq!(
    // sum of all entities in the matrix
    Matrix::<2, 4>::new(1.0).sum(),
    8.0
  );
}

#[test]
fn test_transpose() {
  assert_eq!(
    &Matrix {
      rows: [
        [0.0, 1.0, 2.0],
        [3.0, 4.0, 5.0]
      ]
    }
    .transpose(),
    &Matrix {
      rows: [
        [0.0, 3.0],
        [1.0, 4.0],
        [2.0, 5.0]
      ]
    }
  );
}
