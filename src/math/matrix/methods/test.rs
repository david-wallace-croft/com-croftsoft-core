// =============================================================================
//! - Unit tests for the Matrix methods
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2022-09-04
//! - Updated: 2023-07-26
//!
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

#[cfg(test)]
use super::super::structures::*;

#[test]
fn test_add_matrix() {
  let mut self_matrix = Matrix::<2, 4>::new(1.0);
  assert_eq!(self_matrix.add_matrix(&Matrix::new(1.0)), &Matrix::new(2.0));
  assert_eq!(self_matrix, Matrix::new(2.0));
}

#[test]
fn test_add_scalar() {
  assert_eq!(
    // A 2x4 matrix of all ones
    &Matrix::<2, 4>::new(1.0),
    // The same by adding 1 to the default
    Matrix::default().add_scalar(1.0)
  );
}

#[test]
fn test_divide_by_matrix_entrywise() {
  let mut self_matrix_1x4 = Matrix {
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
    self_matrix_1x4.divide_by_matrix_entrywise(&divisor_matrix_1x4),
    &expected_quotient_matrix_1x4
  );
  assert_eq!(self_matrix_1x4, expected_quotient_matrix_1x4);
}

#[test]
fn test_divide_by_scalar() {
  let dividend = &mut Matrix::<1, 1>::new(6.0);
  assert_eq!(dividend.divide_by_scalar(3.0), &Matrix::<1, 1>::new(2.0));
  assert_eq!(dividend, &Matrix::<1, 1>::new(2.0));
}

#[test]
fn test_get_entry() {
  // first row, last column
  let indices = Indices {
    row: 0,
    column: 3,
  };
  assert_eq!(
    Matrix::<2, 4>::default()
      .set_entry(indices, 1.0)
      .get_entry(indices),
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
    &[
      0.0, 0.0, 0.0, 1.0
    ]
  );
}

#[test]
fn test_is_square() {
  assert!(Matrix::<1, 1>::default().is_square());
  assert!(!Matrix::<1, 2>::default().is_square());
}

#[test]
fn test_matches_closely() {
  assert!(
    Matrix::<2, 4>::new(1.0 / 3.0).matches_closely(&Matrix::new(0.33), 0.01)
  );
  assert!(Matrix::<2, 4>::new(0.0).matches_exactly(&Matrix::default()));
}

#[test]
fn test_multiply_with_matrix() {
  let mut self_multiplicand = Matrix::<2, 4>::new(2.0);
  let multiplier = Matrix::<4, 4>::new(3.0);
  let expected_product = Matrix::<2, 4>::new(24.0);
  assert_eq!(
    self_multiplicand.multiply_with_matrix(&multiplier),
    &expected_product
  );
  assert_eq!(self_multiplicand, expected_product);
  let mut self_multiplicand = Matrix {
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
  let multiplier = Matrix {
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
  let expected_product = Matrix {
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
    self_multiplicand.multiply_with_matrix(&multiplier),
    &expected_product
  );
  assert_eq!(self_multiplicand, expected_product);
}

#[test]
fn test_multiply_with_matrix_entrywise() {
  let mut self_matrix_1x4 = Matrix {
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
    self_matrix_1x4.multiply_with_matrix_entrywise(&weighting_matrix_1x4),
    &expected_hadamard_product_1x4
  );
  assert_eq!(self_matrix_1x4, expected_hadamard_product_1x4);
}

#[test]
fn test_multiply_with_scalar() {
  let mut self_matrix = Matrix::<1, 1>::new(3.0);
  let expected_product = Matrix::<1, 1>::new(15.0);
  assert_eq!(self_matrix.multiply_with_scalar(5.0), &expected_product);
  assert_eq!(self_matrix, expected_product);
}

#[test]
fn test_negate() {
  let mut self_matrix = Matrix::<1, 1>::new(1.0);
  let expected_negated = Matrix::<1, 1>::new(-1.0);
  assert_eq!(self_matrix.negate(), &expected_negated);
  assert_eq!(self_matrix, expected_negated);
}

#[test]
fn test_submatrix() {
  assert_eq!(
    Matrix {
      rows: [
        [
          0.0, 1.0, 2.0
        ],
        [
          3.0, 4.0, 5.0
        ]
      ]
    }
    .submatrix::<1, 2>(Indices {
      row: 1,
      column: 0
    }),
    Matrix {
      rows: [[
        3.0, 4.0
      ]]
    }
  );
}

#[test]
fn test_subtract_from_scalar() {
  let minuend = 3.0;
  let mut self_matrix = Matrix::<1, 1>::new(2.0);
  let expected_difference = Matrix::new(1.0);
  assert_eq!(
    self_matrix.subtract_from_scalar(minuend),
    &expected_difference,
  );
  assert_eq!(self_matrix, expected_difference);
}

#[test]
fn test_subtract_matrix() {
  let mut self_matrix = Matrix::<1, 1>::new(3.0);
  let subtrahend = Matrix::new(2.0);
  let expected_difference = Matrix::new(1.0);
  assert_eq!(
    self_matrix.subtract_matrix(&subtrahend),
    &expected_difference,
  );
  assert_eq!(self_matrix, expected_difference);
}

#[test]
fn test_subtract_scalar() {
  assert_eq!(
    // A 2x4 matrix of negative ones
    &Matrix::<2, 4>::new(-1.0),
    // The same by subtracting one
    Matrix::default().subtract_scalar(1.0)
  );
}

#[test]
fn test_sum_entries() {
  assert_eq!(
    // sum of all entities in the matrix
    Matrix::<2, 4>::new(1.0).sum_entries(),
    8.0
  );
}

#[test]
fn test_transpose() {
  assert_eq!(
    &Matrix {
      rows: [
        [
          0.0, 1.0, 2.0
        ],
        [
          3.0, 4.0, 5.0
        ]
      ]
    }
    .transpose(),
    &Matrix {
      rows: [
        [
          0.0, 3.0
        ],
        [
          1.0, 4.0
        ],
        [
          2.0, 5.0
        ]
      ]
    }
  );
}
