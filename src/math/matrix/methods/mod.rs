// =============================================================================
//! - Methods for the structure Matrix
//!
//! # Metadata
//! - Copyright: &copy; 1998 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-09-29
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

#[cfg(test)]
mod test;

use super::structures::*;

// Methods ---------------------------------------------------------------------

impl<const R: usize, const C: usize> Matrix<R, C> {
  // ---------------------------------------------------------------------------
  /// Adds the argument entries to all corresponding entries and returns self
  // ---------------------------------------------------------------------------
  pub fn add_matrix(
    &mut self,
    addend: &Self,
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
  /// Divides corresponding entries and then returns a reference to self
  // ---------------------------------------------------------------------------
  pub fn divide_by_matrix_entrywise(
    &mut self,
    divisor: &Self,
  ) -> &mut Self {
    for r in 0..R {
      for c in 0..C {
        self.rows[r][c] /= divisor.rows[r][c];
      }
    }
    self
  }

  // ---------------------------------------------------------------------------
  /// Divides each entry by the argument and then returns a reference to self
  // ---------------------------------------------------------------------------
  pub fn divide_by_scalar(
    &mut self,
    divisor: f64,
  ) -> &mut Self {
    for r in 0..R {
      for c in 0..C {
        self.rows[r][c] /= divisor;
      }
    }
    self
  }

  // ---------------------------------------------------------------------------
  /// Returns the entry at the position given by the indices
  // ---------------------------------------------------------------------------
  pub fn get_entry(
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
  /// Returns false if any difference magnitude is greater than the tolerance.
  ///
  /// The tolerance should be a positive number.
  // ---------------------------------------------------------------------------
  pub fn matches_closely(
    &self,
    other: &Self,
    tolerance: f64,
  ) -> bool {
    for r in 0..R {
      for c in 0..C {
        let difference_magnitude = (self.rows[r][c] - other.rows[r][c]).abs();
        if difference_magnitude > tolerance {
          return false;
        }
      }
    }
    true
  }

  // ---------------------------------------------------------------------------
  /// Returns true if the other Matrix has the exact same entries
  // ---------------------------------------------------------------------------
  pub fn matches_exactly(
    &self,
    other: &Self,
  ) -> bool {
    for r in 0..R {
      for c in 0..C {
        if self.rows[r][c] != other.rows[r][c] {
          return false;
        }
      }
    }
    true
  }

  // ---------------------------------------------------------------------------
  /// Multiplies with a square matrix and then returns a reference to self
  // ---------------------------------------------------------------------------
  pub fn multiply_with_matrix(
    &mut self,
    multiplier: &Matrix<C, C>,
  ) -> &mut Self {
    let product = Self::multiply_matrix_with_matrix(self, multiplier);
    for r in 0..R {
      for c in 0..C {
        self.rows[r][c] = product.rows[r][c];
      }
    }
    self
  }

  // ---------------------------------------------------------------------------
  /// Multiplies corresponding entries and then returns a reference to self
  ///
  /// This result is known as the Hadamard Product:<br>
  /// <https://en.wikipedia.org/wiki/Hadamard_product_(matrices)>
  // ---------------------------------------------------------------------------
  pub fn multiply_with_matrix_entrywise(
    &mut self,
    weighting_matrix: &Self,
  ) -> &mut Self {
    for r in 0..R {
      for c in 0..C {
        self.rows[r][c] *= weighting_matrix.rows[r][c];
      }
    }
    self
  }

  // ---------------------------------------------------------------------------
  /// Multiplies all entries by the scalar and then returns a reference to self
  // ---------------------------------------------------------------------------
  pub fn multiply_with_scalar(
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
  /// Multiplies all entries by -1.0 and then returns a reference to self
  // ---------------------------------------------------------------------------
  pub fn negate(&mut self) -> &mut Self {
    for r in 0..R {
      for c in 0..C {
        self.rows[r][c] *= -1.0;
      }
    }
    self
  }

  // ---------------------------------------------------------------------------
  /// Sets the entry at the position given by the indices and then returns self
  // ---------------------------------------------------------------------------
  pub fn set_entry(
    &mut self,
    indices: Indices,
    value: f64,
  ) -> &mut Self {
    self.rows[indices.row][indices.column] = value;
    self
  }

  // ---------------------------------------------------------------------------
  /// Returns a new Matrix that is a submatrix of self
  // ---------------------------------------------------------------------------
  pub fn submatrix<const P: usize, const K: usize>(
    &self,
    offset_indices: Indices,
  ) -> Matrix<P, K> {
    let mut submatrix: Matrix<P, K> = Matrix::default();
    let offset_row: usize = offset_indices.row;
    let offset_column: usize = offset_indices.column;
    for row in 0..P {
      for column in 0..K {
        submatrix.rows[row][column] =
          self.rows[row + offset_row][column + offset_column];
      }
    }
    submatrix
  }

  // ---------------------------------------------------------------------------
  /// Subtracts all entries from the scalar and then returns a reference to self
  // ---------------------------------------------------------------------------
  pub fn subtract_from_scalar(
    &mut self,
    minuend: f64,
  ) -> &mut Self {
    for r in 0..R {
      for c in 0..C {
        self.rows[r][c] = minuend - self.rows[r][c];
      }
    }
    self
  }

  // ---------------------------------------------------------------------------
  /// Subtracts the argument entries from corresponding entries and returns self
  // ---------------------------------------------------------------------------
  pub fn subtract_matrix(
    &mut self,
    subtrahend: &Self,
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
  pub fn sum_entries(&self) -> f64 {
    self.rows.iter().fold(0.0, |sum, row| {
      sum + row.iter().fold(0.0, |sum, entry| sum + entry)
    })
  }

  // ---------------------------------------------------------------------------
  /// Returns a new Matrix with the rows and columns switched.
  // ---------------------------------------------------------------------------
  pub fn transpose(&self) -> Matrix<C, R> {
    let mut transposed_matrix = Matrix::<C, R>::default();
    for (row_index, row) in self.rows.iter().enumerate() {
      for (column_index, entry) in row.iter().enumerate() {
        transposed_matrix.rows[column_index][row_index] = *entry;
      }
    }
    transposed_matrix
  }
}
