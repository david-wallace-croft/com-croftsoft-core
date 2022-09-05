// =============================================================================
//! - A mathematical matrix structure
//!
//! # Metadata
//! - Copyright: &copy; 1998 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-09-05
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
///   &Matrix::<2, 4>::new(1.0),        // A 2x4 matrix of all ones
///   Matrix::default().add(1.0));      // The same by adding one to the default
/// let indices = Indices { row: 0, column: 3 }; // first row, last column
/// assert_eq!(
///   Matrix::<2, 4>::default().set(indices, 1.0).get(indices), // set and get
///   1.0);
/// assert_eq!(
///   Matrix::<2, 4>::default().set(indices, 1.0).get_row(0), // set and get_row
///   &[0.0, 0.0, 0.0, 1.0]);
/// assert_eq!(
///   Matrix::<2, 4>::new(1.0).sum(),
///   8.0);
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
  pub rows: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Default for Matrix<R, C> {
  // -----------------------------------------------------------------------------
  /// Makes a new Matrix of all zero entries
  // -----------------------------------------------------------------------------
  fn default() -> Self {
    Self {
      rows: [[0.0; C]; R],
    }
  }
}

impl<const R: usize, const C: usize> Matrix<R, C> {
  // -----------------------------------------------------------------------------
  /// Adds the argument to all entries in the matrix
  // -----------------------------------------------------------------------------
  pub fn add(
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
  /// Calculates the sum of all of the entries in the Matrix
  // ---------------------------------------------------------------------------
  pub fn sum(&self) -> f64 {
    self.rows.iter().fold(0.0, |sum, row| {
      sum + row.iter().fold(0.0, |sum, entry| sum + entry)
    })
  }
}
