#[cfg(test)]
use super::*;

#[test]
fn test_add() {
  assert_eq!(1.0 + &Matrix::<1, 1>::new(2.0), Matrix::new(3.0));
  assert_eq!(1.0 + Matrix::<1, 1>::new(2.0), Matrix::new(3.0));
  assert_eq!(&Matrix::<1, 1>::new(1.0) + 2.0, Matrix::new(3.0));
  assert_eq!(Matrix::<1, 1>::new(1.0) + 2.0, Matrix::new(3.0));
  assert_eq!(
    &Matrix::<1, 1>::new(1.0) + &Matrix::new(2.0),
    Matrix::new(3.0)
  );
  assert_eq!(
    Matrix::<1, 1>::new(1.0) + &Matrix::new(2.0),
    Matrix::new(3.0)
  );
  assert_eq!(
    Matrix::<1, 1>::new(1.0) + Matrix::new(2.0),
    Matrix::new(3.0)
  );
  assert_eq!(
    &Matrix::<1, 1>::new(1.0) + Matrix::new(2.0),
    Matrix::new(3.0)
  );
}

#[test]
fn test_add_assign() {
  let matrix_3 = Matrix::<1, 1>::new(3.0);
  let mut matrix = Matrix::<1, 1>::new(1.0);
  matrix += 2.0;
  assert_eq!(matrix, matrix_3);
  let mut matrix = Matrix::<1, 1>::new(1.0);
  matrix += Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, matrix_3);
  let mut matrix = Matrix::<1, 1>::new(1.0);
  matrix += &Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, matrix_3);
  let mut matrix = &mut Matrix::<1, 1>::new(1.0);
  matrix += Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, &matrix_3);
  let mut matrix = &mut Matrix::<1, 1>::new(1.0);
  matrix += &Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, &matrix_3);
}

#[test]
fn test_add_matrix_to_matrix() {
  assert_eq!(
    add_matrix_to_matrix(&Matrix::<1, 1>::new(1.0), &Matrix::new(2.0)),
    Matrix::new(3.0)
  );
}

#[test]
fn test_add_matrix_to_scalar() {
  assert_eq!(
    add_matrix_to_scalar(&Matrix::<1, 1>::new(1.0), 2.0),
    Matrix::new(3.0)
  );
}

#[test]
fn test_add_matrix() {
  assert_eq!(
    Matrix::<2, 4>::new(1.0).add_matrix(&Matrix::new(1.0)), // matrix addition
    &Matrix::new(2.0)
  );
}

#[test]
fn test_add_scalar() {
  assert_eq!(
    &Matrix::<2, 4>::new(1.0), // A 2x4 matrix of all ones
    Matrix::default().add_scalar(1.0)
  ); // The same by adding 1 to the default
}

#[test]
fn test_default() {
  assert_eq!(
    &Matrix::<2, 4>::default(), // A "two by four" matrix of all zeroes
    &Matrix {
      rows: [[0.0; 4]; 2]
    }
  ); // A matrix of two rows and four columns
}

#[test]
fn test_get() {
  let indices = Indices {
    row: 0,
    column: 3,
  }; // first row, last column
  assert_eq!(
    Matrix::<2, 4>::default().set_entry(indices, 1.0).get_entry(indices),
    1.0
  );
}

#[test]
fn test_get_row() {
  let indices = Indices {
    row: 0,
    column: 3,
  }; // first row, last column
  assert_eq!(
    Matrix::<2, 4>::default().set_entry(indices, 1.0).get_row(0),
    &[0.0, 0.0, 0.0, 1.0]
  );
}

#[test]
fn test_identity() {
  assert_eq!(
    &identity(),
    &Matrix {
      rows: [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0]
      ]
    }
  );
  assert_eq!(identity::<3>().sum(), 3.0);
  assert_eq!(
    Matrix {
      rows: [
        [1.0, 2.0],
        [3.0, 4.0]
      ]
    }
    .multiply_matrix(&identity()),
    Matrix {
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
fn test_multiply_entries() {
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
    matrix_1x4.multiply_entries(&weighting_matrix),
    &expected_hadamard_product
  );
}

#[test]
fn test_multiply_matrix() {
  assert_eq!(
    &Matrix::<2, 4>::new(2.0).multiply_matrix(&Matrix::<4, 3>::new(3.0)),
    &Matrix::<2, 3>::new(24.0)
  );
  let matrix_multiplicand = Matrix {
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
    &matrix_multiplicand.multiply_matrix(&matrix_multiplier),
    &expected_matrix_product
  );
}

#[test]
fn test_multiply_scalar() {
  assert_eq!(
    Matrix::<2, 4>::new(3.0).multiply_scalar(5.0),
    &Matrix::new(15.0)
  );
}

#[test]
fn test_neg() {
  assert_eq!(-Matrix::<1, 1>::new(1.0), Matrix::new(-1.0));
  assert_eq!(-&Matrix::<1, 1>::new(1.0), Matrix::new(-1.0));
}

#[test]
fn test_negate_matrix() {
  assert_eq!(negate(&Matrix::<1, 1>::new(1.0)), Matrix::new(-1.0));
}

#[test]
fn test_negate() {
  assert_eq!(Matrix::<1, 1>::new(1.0).negate(), &Matrix::new(-1.0));
}

#[test]
fn test_new() {
  assert_eq!(
    &Matrix::<2, 4>::new(0.0), // A 2x4 matrix from a new() constructor
    &Matrix::default()
  ); // The same with the dimensions inferred
}

#[test]
fn test_sub() {
  assert_eq!(&Matrix::<1, 1>::new(3.0) - 2.0, Matrix::new(1.0));
  assert_eq!(Matrix::<1, 1>::new(3.0) - 2.0, Matrix::new(1.0));
  assert_eq!(
    &Matrix::<1, 1>::new(3.0) - &Matrix::new(2.0),
    Matrix::new(1.0)
  );
  assert_eq!(
    Matrix::<1, 1>::new(3.0) - &Matrix::new(2.0),
    Matrix::new(1.0)
  );
  assert_eq!(
    Matrix::<1, 1>::new(3.0) - Matrix::new(2.0),
    Matrix::new(1.0)
  );
  assert_eq!(
    &Matrix::<1, 1>::new(3.0) - Matrix::new(2.0),
    Matrix::new(1.0)
  );
}

#[test]
fn test_sub_assign() {
  let matrix_1 = Matrix::<1, 1>::new(1.0);
  let mut matrix = Matrix::<1, 1>::new(3.0);
  matrix -= 2.0;
  assert_eq!(matrix, matrix_1);
  let mut matrix = Matrix::<1, 1>::new(3.0);
  matrix -= Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, matrix_1);
  let mut matrix = Matrix::<1, 1>::new(3.0);
  matrix -= &Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, matrix_1);
  let mut matrix = &mut Matrix::<1, 1>::new(3.0);
  matrix -= Matrix::<1, 1>::new(2.0);
  assert_eq!(matrix, &matrix_1);
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
  ); // matrix subtraction
}

#[test]
fn test_subtract_matrix_from_matrix() {
  assert_eq!(
    subtract_matrix_from_matrix(&Matrix::<1, 1>::new(3.0), &Matrix::new(2.0)),
    Matrix::new(1.0)
  );
}

#[test]
fn test_subtract_scalar() {
  assert_eq!(
    &Matrix::<2, 4>::new(-1.0), // A 2x4 matrix of negative ones
    Matrix::default().subtract_scalar(1.0)
  ); // The same by subtracting one
}

#[test]
fn test_subtract_scalar_from_matrix() {
  assert_eq!(
    subtract_scalar_from_matrix(&Matrix::<1, 1>::new(3.0), 2.0),
    Matrix::new(1.0)
  );
}

#[test]
fn test_sum() {
  assert_eq!(
    Matrix::<2, 4>::new(1.0).sum(), // sum of all entities in the matrix
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
