// =============================================================================
//! - Functions for the structure Matrix
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java updated: 1998-12-27
//! - Rust created: 2022-09-04
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the Java class com.croftsoft.core.math.Matrix
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

#[cfg(test)]
mod test;

use super::structures::*;

// Associated functions --------------------------------------------------------

impl<const R: usize, const C: usize> Matrix<R, C> {
  // ---------------------------------------------------------------------------
  /// Adds the arguments and return the sum as a new Matrix
  // ---------------------------------------------------------------------------
  pub fn add_matrix_with_matrix(
    augend: &Self,
    addend: &Self,
  ) -> Self {
    let mut sum = Self::default();
    for r in 0..R {
      for c in 0..C {
        sum.rows[r][c] = augend.rows[r][c] + addend.rows[r][c];
      }
    }
    sum
  }

  // ---------------------------------------------------------------------------
  /// Adds the arguments and then returns the sum as a new Matrix
  // ---------------------------------------------------------------------------
  pub fn add_matrix_with_scalar(
    augend: &Self,
    addend: f64,
  ) -> Self {
    let mut sum = Self::new(addend);
    for r in 0..R {
      for c in 0..C {
        sum.rows[r][c] += augend.rows[r][c];
      }
    }
    sum
  }

  // ---------------------------------------------------------------------------
  /// Divides corresponding entries and returns the quotient as a new Matrix
  // ---------------------------------------------------------------------------
  pub fn divide_matrix_by_matrix_entrywise(
    dividend_matrix: &Self,
    divisor_matrix: &Self,
  ) -> Self {
    let mut quotient_matrix = Self::default();
    for r in 0..R {
      for c in 0..C {
        quotient_matrix.rows[r][c] =
          dividend_matrix.rows[r][c] / divisor_matrix.rows[r][c];
      }
    }
    quotient_matrix
  }

  // ---------------------------------------------------------------------------
  /// Divides each entry by the scalar and then returns a new Matrix
  // ---------------------------------------------------------------------------
  pub fn divide_matrix_by_scalar(
    dividend: &Self,
    divisor: f64,
  ) -> Self {
    let mut quotient = Self::default();
    for r in 0..R {
      for c in 0..C {
        quotient.rows[r][c] = dividend.rows[r][c] / divisor;
      }
    }
    quotient
  }

  // ---------------------------------------------------------------------------
  /// Multiplies the arguments and then returns the product as a new Matrix
  // ---------------------------------------------------------------------------
  pub fn multiply_matrix_with_matrix<const K: usize>(
    multiplicand: &Self,
    multiplier: &Matrix<C, K>,
  ) -> Matrix<R, K> {
    let mut product = Matrix::<R, K>::default();
    for r in 0..R {
      for k in 0..K {
        for i in 0..C {
          product.rows[r][k] += multiplicand.rows[r][i] * multiplier.rows[i][k];
        }
      }
    }
    product
  }

  // ---------------------------------------------------------------------------
  /// Multiplies entries and returns the Hadamard product as a new Matrix
  ///
  /// <https://en.wikipedia.org/wiki/Hadamard_product_(matrices)>
  // ---------------------------------------------------------------------------
  pub fn multiply_matrix_with_matrix_entrywise(
    original_matrix: &Self,
    weighting_matrix: &Self,
  ) -> Self {
    let mut hadamard_product = Self::default();
    for r in 0..R {
      for c in 0..C {
        hadamard_product.rows[r][c] =
          original_matrix.rows[r][c] * weighting_matrix.rows[r][c];
      }
    }
    hadamard_product
  }

  pub fn multiply_matrix_with_scalar(
    multiplicand: &Self,
    multiplier: f64,
  ) -> Self {
    let mut product = Self::new(multiplier);
    for r in 0..R {
      for c in 0..C {
        product.rows[r][c] *= multiplicand.rows[r][c];
      }
    }
    product
  }

  // ---------------------------------------------------------------------------
  /// Multiplies all entries by -1.0 and then returns the new negated Matrix
  // ---------------------------------------------------------------------------
  pub fn negate_matrix(matrix: &Self) -> Self {
    let mut negated_matrix = Self::default();
    for r in 0..R {
      for c in 0..C {
        negated_matrix.rows[r][c] = -matrix.rows[r][c];
      }
    }
    negated_matrix
  }

  // ---------------------------------------------------------------------------
  /// Makes a new Matrix with all entries set to the argument
  // ---------------------------------------------------------------------------
  pub fn new(value: f64) -> Self {
    Self {
      rows: [[value; C]; R],
    }
  }

  // ---------------------------------------------------------------------------
  /// Subtracts the 2nd from the 1st and returns the difference as a new Matrix
  // ---------------------------------------------------------------------------
  pub fn subtract_matrix_from_matrix(
    minuend: &Self,
    subtrahend: &Self,
  ) -> Self {
    let mut difference = Self::default();
    for r in 0..R {
      for c in 0..C {
        difference.rows[r][c] = minuend.rows[r][c] - subtrahend.rows[r][c];
      }
    }
    difference
  }

  // ---------------------------------------------------------------------------
  /// Subtracts the 2nd from the 1st and returns the difference as a new Matrix
  // ---------------------------------------------------------------------------
  pub fn subtract_matrix_from_scalar(
    minuend: f64,
    subtrahend: &Self,
  ) -> Self {
    let mut difference = Self::new(minuend);
    for r in 0..R {
      for c in 0..C {
        difference.rows[r][c] -= subtrahend.rows[r][c];
      }
    }
    difference
  }

  // ---------------------------------------------------------------------------
  /// Subtracts the 2nd from the 1st and returns the difference as a new Matrix
  // ---------------------------------------------------------------------------
  pub fn subtract_scalar_from_matrix(
    minuend: &Self,
    subtrahend: f64,
  ) -> Self {
    let mut difference = Self::default();
    for r in 0..R {
      for c in 0..C {
        difference.rows[r][c] = minuend.rows[r][c] - subtrahend;
      }
    }
    difference
  }
}

// Associated functions for a square Matrix ------------------------------------

impl<const R: usize> Matrix<R, R> {
  // ---------------------------------------------------------------------------
  /// Makes a square matrix with the diagonal values set to 1.0 and all others 0
  // ---------------------------------------------------------------------------
  pub fn identity() -> Self {
    let mut identity_matrix = Self::default();
    for r in 0..R {
      identity_matrix.rows[r][r] = 1.0;
    }
    identity_matrix
  }
}

// Associated functions for a rotation Matrix ----------------------------------

impl Matrix<3, 3> {
  pub fn to_rotation_matrix_x_from_degrees(degrees: Degrees) -> Self {
    Self::to_rotation_matrix_x_from_radians(degrees.into())
  }

  pub fn to_rotation_matrix_x_from_radians(radians: Radians) -> Self {
    let cos = radians.0.cos();
    let sin = radians.0.sin();
    Matrix {
      rows: [
        [
          1.0, 0.0, 0.0,
        ],
        [
          0.0, cos, -sin,
        ],
        [
          0.0, sin, cos,
        ],
      ],
    }
  }

  pub fn to_rotation_matrix_y_from_degrees(degrees: Degrees) -> Self {
    Self::to_rotation_matrix_y_from_radians(degrees.into())
  }

  pub fn to_rotation_matrix_y_from_radians(radians: Radians) -> Self {
    let cos = radians.0.cos();
    let sin = radians.0.sin();
    Matrix {
      rows: [
        [
          cos, 0.0, sin,
        ],
        [
          0.0, 1.0, 0.0,
        ],
        [
          -sin, 0.0, cos,
        ],
      ],
    }
  }

  pub fn to_rotation_matrix_z_from_degrees(degrees: Degrees) -> Self {
    Self::to_rotation_matrix_z_from_radians(degrees.into())
  }

  pub fn to_rotation_matrix_z_from_radians(radians: Radians) -> Self {
    let cos = radians.0.cos();
    let sin = radians.0.sin();
    Matrix {
      rows: [
        [
          cos, -sin, 0.0,
        ],
        [
          sin, cos, 0.0,
        ],
        [
          0.0, 0.0, 1.0,
        ],
      ],
    }
  }
}
