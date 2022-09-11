// =============================================================================
//! - A mathematical matrix structure
//!
//! # Metadata
//! - Copyright: &copy; 1998 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-09-11
//! - Rust since: 2022-09-04
//! - Java version: 1998-12-27
//!
//! # History
//! - Adapted from the Java class com.croftsoft.core.math.Matrix
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

// -----------------------------------------------------------------------------
/// The row and column indices of a Matrix, indexed from zero
// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Indices {
  pub row: usize,
  pub column: usize,
}

// -----------------------------------------------------------------------------
/// A mathematical matrix structure
///
/// # Examples
/// ```
/// use com_croftsoft_core::math::matrix::*;
/// assert_eq!(
///   &Matrix::<2, 4>::default(),       // A "two by four" matrix of all zeroes
///   &Matrix { rows: [[0.0; 4]; 2] }); // A matrix of two rows and four columns
/// assert_eq!(
///   &Matrix::<2, 4>::new(0.0),        // A 2x4 matrix from a new() constructor
///   &Matrix::default());              // The same with the dimensions inferred
/// assert_eq!(
///   &Matrix::<2, 4>::new(1.0),          // A 2x4 matrix of all ones
///   Matrix::default().add_scalar(1.0)); // The same by adding 1 to the default
/// assert_eq!(
///   Matrix::<2, 4>::new(1.0).add_matrix(Matrix::new(1.0)), // matrix addition
///   &Matrix::new(2.0));
/// assert_eq!(
///   &Matrix::<2, 4>::new(2.0).multiply_matrix(Matrix::<4, 3>::new(3.0)),
///   &Matrix::<2, 3>::new(24.0));
/// let matrix_multiplicand = Matrix {
///   rows: [[1.0, 0.0, 1.0],
///          [2.0, 1.0, 1.0],
///          [0.0, 1.0, 1.0],
///          [1.0, 1.0, 2.0]],
/// };
/// let matrix_multiplier = Matrix {
///   rows: [[1.0, 2.0, 1.0],
///          [2.0, 3.0, 1.0],
///          [4.0, 2.0, 2.0]],
/// };
/// let expected_matrix_product = Matrix {
///   rows: [[5.0, 4.0, 3.0],
///          [8.0, 9.0, 5.0],
///          [6.0, 5.0, 3.0],
///          [11.0, 9.0, 6.0]],
/// };
/// assert_eq!(
///   &matrix_multiplicand.multiply_matrix(matrix_multiplier),
///   &expected_matrix_product);
/// assert_eq!(
///   Matrix::<2, 4>::new(3.0).multiply_scalar(5.0),
///   &Matrix::new(15.0));
/// let indices = Indices { row: 0, column: 3 }; // first row, last column
/// assert_eq!(
///   Matrix::<2, 4>::default().set(indices, 1.0).get(indices), // set and get
///   1.0);
/// assert_eq!(
///   Matrix::<2, 4>::default().set(indices, 1.0).get_row(0), // set and get_row
///   &[0.0, 0.0, 0.0, 1.0]);
/// assert_eq!(
///   &Matrix::<2, 4>::new(-1.0),              // A 2x4 matrix of negative ones
///   Matrix::default().subtract_scalar(1.0)); // The same by subtracting one
/// assert_eq!(
///   Matrix::<2, 4>::new(3.0).subtract_matrix(Matrix::new(2.0)),
///   &Matrix::new(1.0));                      // matrix subtraction
/// assert_eq!(
///   Matrix::<2, 4>::new(1.0).sum(), // sum of all entities in the matrix
///   8.0);
/// assert_eq!(
///   &Matrix { rows: [[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]] }.transpose(),
///   &Matrix { rows: [[0.0, 3.0], [1.0, 4.0], [2.0, 5.0]] });
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
  pub rows: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Default for Matrix<R, C> {
  // ---------------------------------------------------------------------------
  /// Makes a new Matrix of all zero entries
  // ---------------------------------------------------------------------------
  fn default() -> Self {
    Self {
      rows: [[0.0; C]; R],
    }
  }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
  // ---------------------------------------------------------------------------
  /// Adds the argument entries to all corresponding entries and returns self
  // ---------------------------------------------------------------------------
  pub fn add_matrix(
    &mut self,
    addend: Self,
  ) -> &mut Self {
    for r in 0..R {
      for c in 0..C {
        self.rows[r][c] += addend.rows[r][c];
      }
    }
    self
  }

  // ---------------------------------------------------------------------------
  /// Adds the scalar to all entries and then returns a reference to self
  // ---------------------------------------------------------------------------
  pub fn add_scalar(
    &mut self,
    addend: f64,
  ) -> &mut Self {
    for r in 0..R {
      for c in 0..C {
        self.rows[r][c] += addend;
      }
    }
    self
  }

  // ---------------------------------------------------------------------------
  /// Returns the entry at the position given by the indices
  // ---------------------------------------------------------------------------
  pub fn get(
    &self,
    indices: Indices,
  ) -> f64 {
    self.rows[indices.row][indices.column]
  }

  // ---------------------------------------------------------------------------
  /// Returns a reference to a row of entries, indexed from zero
  // ---------------------------------------------------------------------------
  pub fn get_row(
    &self,
    row_index: usize,
  ) -> &[f64; C] {
    &self.rows[row_index]
  }

  // ---------------------------------------------------------------------------
  /// Multiplies by the Matrix multiplier and returns the Matrix product
  // ---------------------------------------------------------------------------
  pub fn multiply_matrix<const K: usize>(
    &self,
    multiplier: Matrix<C, K>,
  ) -> Matrix<R, K> {
    let mut matrix_product = Matrix::<R, K>::default();
    for r in 0..R {
      for k in 0..K {
        for i in 0..C {
          matrix_product.rows[r][k] += self.rows[r][i] * multiplier.rows[i][k];
        }
      }
    }
    matrix_product
  }

  // ---------------------------------------------------------------------------
  /// Multiplies all entries by the scalar then returns a reference to self
  // ---------------------------------------------------------------------------
  pub fn multiply_scalar(
    &mut self,
    multiplier: f64,
  ) -> &mut Self {
    for r in 0..R {
      for c in 0..C {
        self.rows[r][c] *= multiplier;
      }
    }
    self
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
  /// Sets the entry at the position given by the indices and then returns self
  // ---------------------------------------------------------------------------
  pub fn set(
    &mut self,
    indices: Indices,
    value: f64,
  ) -> &mut Self {
    self.rows[indices.row][indices.column] = value;
    self
  }

  // ---------------------------------------------------------------------------
  /// Subtracts the argument entries from corresponding entries and returns self
  // ---------------------------------------------------------------------------
  pub fn subtract_matrix(
    &mut self,
    subtrahend: Self,
  ) -> &mut Self {
    for r in 0..R {
      for c in 0..C {
        self.rows[r][c] -= subtrahend.rows[r][c];
      }
    }
    self
  }

  // ---------------------------------------------------------------------------
  /// Subtracts the scalar from all entries and then returns a reference to self
  // ---------------------------------------------------------------------------
  pub fn subtract_scalar(
    &mut self,
    subtrahend: f64,
  ) -> &mut Self {
    for r in 0..R {
      for c in 0..C {
        self.rows[r][c] -= subtrahend;
      }
    }
    self
  }

  // ---------------------------------------------------------------------------
  /// Calculates the sum of all of the entries in the Matrix
  // ---------------------------------------------------------------------------
  pub fn sum(&self) -> f64 {
    self.rows.iter().fold(0.0, |sum, row| {
      sum + row.iter().fold(0.0, |sum, entry| sum + entry)
    })
  }

  // ---------------------------------------------------------------------------
  /// Returns a new Matrix with the rows and columns switched.
  // ---------------------------------------------------------------------------
  pub fn transpose(&self) -> Matrix<C, R> {
    let mut transposed_matrix = Matrix {
      rows: [[0.0; R]; C],
    };
    for (row_index, row) in self.rows.iter().enumerate() {
      for (column_index, entry) in row.iter().enumerate() {
        transposed_matrix.rows[column_index][row_index] = *entry;
      }
    }
    transposed_matrix
  }
}

// -----------------------------------------------------------------------------
/// Makes a square matrix with the diagonal values set to 1.0 and all others 0.0
///
/// # Examples
/// ```
/// use com_croftsoft_core::math::matrix::*;
/// assert_eq!(
///   &identity(),
///   &Matrix { rows: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] });
/// assert_eq!(
///   identity::<3>().sum(),
///   3.0);
/// assert_eq!(
///   Matrix { rows: [[1.0, 2.0], [3.0, 4.0]] }.multiply_matrix(identity()),
///   Matrix { rows: [[1.0, 2.0], [3.0, 4.0]] });
/// ```
// -----------------------------------------------------------------------------
pub fn identity<const R: usize>() -> Matrix<R, R> {
  let mut identity_matrix = Matrix::<R, R>::default();
  for r in 0..R {
    identity_matrix.rows[r][r] = 1.0;
  }
  identity_matrix
}
