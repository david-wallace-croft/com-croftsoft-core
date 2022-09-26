// =============================================================================
//! - A mathematical matrix that uses const generics for the rows and columns
//!
//! # Metadata
//! - Copyright: &copy; 1998 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-09-26
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
mod tests;

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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
// -----------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
  pub rows: [[f64; C]; R],
}

// Default ---------------------------------------------------------------------

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
  /// Adds the arguments and return the sum as a new Matrix
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
  /// Multiplies the arguments and return the product as a new Matrix
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

// Operator Add ----------------------------------------------------------------

impl<const R: usize, const C: usize> Add<Matrix<R, C>> for f64 {
  type Output = Matrix<R, C>;

  fn add(
    self,
    rhs: Matrix<R, C>,
  ) -> Self::Output {
    Matrix::add_matrix_with_scalar(&rhs, self)
  }
}

impl<const R: usize, const C: usize> Add<&Matrix<R, C>> for f64 {
  type Output = Matrix<R, C>;

  fn add(
    self,
    rhs: &Matrix<R, C>,
  ) -> Self::Output {
    Matrix::add_matrix_with_scalar(rhs, self)
  }
}

impl<const R: usize, const C: usize> Add<f64> for Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn add(
    self,
    rhs: f64,
  ) -> Self::Output {
    Self::add_matrix_with_scalar(&self, rhs)
  }
}

impl<const R: usize, const C: usize> Add<f64> for &Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn add(
    self,
    rhs: f64,
  ) -> Self::Output {
    Matrix::add_matrix_with_scalar(self, rhs)
  }
}

impl<const R: usize, const C: usize> Add<Matrix<R, C>> for Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn add(
    self,
    rhs: Matrix<R, C>,
  ) -> Self::Output {
    Self::add_matrix_with_matrix(&self, &rhs)
  }
}

impl<const R: usize, const C: usize> Add<&Matrix<R, C>> for Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn add(
    self,
    rhs: &Matrix<R, C>,
  ) -> Self::Output {
    Self::add_matrix_with_matrix(&self, rhs)
  }
}

impl<const R: usize, const C: usize> Add<Matrix<R, C>> for &Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn add(
    self,
    rhs: Matrix<R, C>,
  ) -> Self::Output {
    Matrix::add_matrix_with_matrix(self, &rhs)
  }
}

impl<'a, 'b, const R: usize, const C: usize> Add<&'b Matrix<R, C>>
  for &'a Matrix<R, C>
{
  type Output = Matrix<R, C>;

  fn add(
    self,
    rhs: &'b Matrix<R, C>,
  ) -> Self::Output {
    Matrix::add_matrix_with_matrix(self, rhs)
  }
}

// Operator AddAssign ----------------------------------------------------------

impl<const R: usize, const C: usize> AddAssign<f64> for Matrix<R, C> {
  fn add_assign(
    &mut self,
    rhs: f64,
  ) {
    self.add_scalar(rhs);
  }
}

impl<const R: usize, const C: usize> AddAssign<f64> for &mut Matrix<R, C> {
  fn add_assign(
    &mut self,
    rhs: f64,
  ) {
    self.add_scalar(rhs);
  }
}

impl<const R: usize, const C: usize> AddAssign<Matrix<R, C>> for Matrix<R, C> {
  fn add_assign(
    &mut self,
    rhs: Matrix<R, C>,
  ) {
    self.add_matrix(&rhs);
  }
}

impl<const R: usize, const C: usize> AddAssign<Matrix<R, C>>
  for &mut Matrix<R, C>
{
  fn add_assign(
    &mut self,
    rhs: Matrix<R, C>,
  ) {
    self.add_matrix(&rhs);
  }
}

impl<const R: usize, const C: usize> AddAssign<&Matrix<R, C>> for Matrix<R, C> {
  fn add_assign(
    &mut self,
    rhs: &Matrix<R, C>,
  ) {
    self.add_matrix(rhs);
  }
}

impl<const R: usize, const C: usize> AddAssign<&Matrix<R, C>>
  for &mut Matrix<R, C>
{
  fn add_assign(
    &mut self,
    rhs: &Matrix<R, C>,
  ) {
    self.add_matrix(rhs);
  }
}

// Operator Mul ----------------------------------------------------------------

impl<const R: usize, const C: usize> Mul<Matrix<R, C>> for f64 {
  type Output = Matrix<R, C>;

  fn mul(
    self,
    rhs: Matrix<R, C>,
  ) -> Self::Output {
    Matrix::multiply_matrix_with_scalar(&rhs, self)
  }
}

impl<const R: usize, const C: usize> Mul<&Matrix<R, C>> for f64 {
  type Output = Matrix<R, C>;

  fn mul(
    self,
    rhs: &Matrix<R, C>,
  ) -> Self::Output {
    Matrix::multiply_matrix_with_scalar(rhs, self)
  }
}

impl<const R: usize, const C: usize> Mul<f64> for Matrix<R, C> {
  type Output = Self;

  fn mul(
    self,
    rhs: f64,
  ) -> Self::Output {
    Self::multiply_matrix_with_scalar(&self, rhs)
  }
}

impl<const R: usize, const C: usize> Mul<f64> for &Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn mul(
    self,
    rhs: f64,
  ) -> Self::Output {
    Matrix::multiply_matrix_with_scalar(self, rhs)
  }
}

impl<const R: usize, const C: usize, const K: usize> Mul<Matrix<C, K>>
  for Matrix<R, C>
{
  type Output = Matrix<R, K>;

  fn mul(
    self,
    rhs: Matrix<C, K>,
  ) -> Self::Output {
    Self::multiply_matrix_with_matrix(&self, &rhs)
  }
}

impl<const R: usize, const C: usize, const K: usize> Mul<&Matrix<C, K>>
  for Matrix<R, C>
{
  type Output = Matrix<R, K>;

  fn mul(
    self,
    rhs: &Matrix<C, K>,
  ) -> Self::Output {
    Self::multiply_matrix_with_matrix(&self, rhs)
  }
}

impl<const R: usize, const C: usize, const K: usize> Mul<Matrix<C, K>>
  for &Matrix<R, C>
{
  type Output = Matrix<R, K>;

  fn mul(
    self,
    rhs: Matrix<C, K>,
  ) -> Self::Output {
    Matrix::multiply_matrix_with_matrix(self, &rhs)
  }
}

impl<'a, 'b, const R: usize, const C: usize, const K: usize>
  Mul<&'b Matrix<C, K>> for &'a Matrix<R, C>
{
  type Output = Matrix<R, K>;

  fn mul(
    self,
    rhs: &'b Matrix<C, K>,
  ) -> Self::Output {
    Matrix::multiply_matrix_with_matrix(self, rhs)
  }
}

// Operator MulAssign ----------------------------------------------------------

impl<const R: usize, const C: usize> MulAssign<f64> for Matrix<R, C> {
  fn mul_assign(
    &mut self,
    rhs: f64,
  ) {
    self.multiply_with_scalar(rhs);
  }
}

impl<const R: usize, const C: usize> MulAssign<f64> for &mut Matrix<R, C> {
  fn mul_assign(
    &mut self,
    rhs: f64,
  ) {
    self.multiply_with_scalar(rhs);
  }
}

impl<const R: usize, const C: usize> MulAssign<Matrix<C, C>> for Matrix<R, C> {
  fn mul_assign(
    &mut self,
    rhs: Matrix<C, C>,
  ) {
    self.multiply_with_matrix(&rhs);
  }
}

impl<const R: usize, const C: usize> MulAssign<&Matrix<C, C>> for Matrix<R, C> {
  fn mul_assign(
    &mut self,
    rhs: &Matrix<C, C>,
  ) {
    self.multiply_with_matrix(rhs);
  }
}

impl<const R: usize, const C: usize> MulAssign<Matrix<C, C>>
  for &mut Matrix<R, C>
{
  fn mul_assign(
    &mut self,
    rhs: Matrix<C, C>,
  ) {
    self.multiply_with_matrix(&rhs);
  }
}

impl<const R: usize, const C: usize> MulAssign<&Matrix<C, C>>
  for &mut Matrix<R, C>
{
  fn mul_assign(
    &mut self,
    rhs: &Matrix<C, C>,
  ) {
    self.multiply_with_matrix(rhs);
  }
}

// Operator Neg ----------------------------------------------------------------

impl<const R: usize, const C: usize> Neg for Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn neg(self) -> Self::Output {
    Self::negate_matrix(&self)
  }
}

impl<const R: usize, const C: usize> Neg for &Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn neg(self) -> Self::Output {
    Matrix::negate_matrix(self)
  }
}

// Operator Sub ----------------------------------------------------------------

impl<const R: usize, const C: usize> Sub<f64> for Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn sub(
    self,
    rhs: f64,
  ) -> Self::Output {
    Self::subtract_scalar_from_matrix(&self, rhs)
  }
}

impl<const R: usize, const C: usize> Sub<f64> for &Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn sub(
    self,
    rhs: f64,
  ) -> Self::Output {
    Matrix::subtract_scalar_from_matrix(self, rhs)
  }
}

impl<const R: usize, const C: usize> Sub<Matrix<R, C>> for f64 {
  type Output = Matrix<R, C>;

  fn sub(
    self,
    rhs: Matrix<R, C>,
  ) -> Self::Output {
    Matrix::subtract_matrix_from_scalar(self, &rhs)
  }
}

impl<const R: usize, const C: usize> Sub<&Matrix<R, C>> for f64 {
  type Output = Matrix<R, C>;

  fn sub(
    self,
    rhs: &Matrix<R, C>,
  ) -> Self::Output {
    Matrix::subtract_matrix_from_scalar(self, rhs)
  }
}

impl<const R: usize, const C: usize> Sub<Matrix<R, C>> for Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn sub(
    self,
    rhs: Matrix<R, C>,
  ) -> Self::Output {
    Self::subtract_matrix_from_matrix(&self, &rhs)
  }
}

impl<const R: usize, const C: usize> Sub<&Matrix<R, C>> for Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn sub(
    self,
    rhs: &Matrix<R, C>,
  ) -> Self::Output {
    Self::subtract_matrix_from_matrix(&self, rhs)
  }
}

impl<const R: usize, const C: usize> Sub<Matrix<R, C>> for &Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn sub(
    self,
    rhs: Matrix<R, C>,
  ) -> Self::Output {
    Matrix::subtract_matrix_from_matrix(self, &rhs)
  }
}

impl<'a, 'b, const R: usize, const C: usize> Sub<&'b Matrix<R, C>>
  for &'a Matrix<R, C>
{
  type Output = Matrix<R, C>;

  fn sub(
    self,
    rhs: &'b Matrix<R, C>,
  ) -> Self::Output {
    Matrix::subtract_matrix_from_matrix(self, rhs)
  }
}

// Operator SubAssign ----------------------------------------------------------

impl<const R: usize, const C: usize> SubAssign<f64> for Matrix<R, C> {
  fn sub_assign(
    &mut self,
    rhs: f64,
  ) {
    self.subtract_scalar(rhs);
  }
}

impl<const R: usize, const C: usize> SubAssign<f64> for &mut Matrix<R, C> {
  fn sub_assign(
    &mut self,
    rhs: f64,
  ) {
    self.subtract_scalar(rhs);
  }
}

impl<const R: usize, const C: usize> SubAssign<Matrix<R, C>> for Matrix<R, C> {
  fn sub_assign(
    &mut self,
    rhs: Matrix<R, C>,
  ) {
    self.subtract_matrix(&rhs);
  }
}

impl<const R: usize, const C: usize> SubAssign<Matrix<R, C>>
  for &mut Matrix<R, C>
{
  fn sub_assign(
    &mut self,
    rhs: Matrix<R, C>,
  ) {
    self.subtract_matrix(&rhs);
  }
}

impl<const R: usize, const C: usize> SubAssign<&Matrix<R, C>> for Matrix<R, C> {
  fn sub_assign(
    &mut self,
    rhs: &Matrix<R, C>,
  ) {
    self.subtract_matrix(rhs);
  }
}

impl<const R: usize, const C: usize> SubAssign<&Matrix<R, C>>
  for &mut Matrix<R, C>
{
  fn sub_assign(
    &mut self,
    rhs: &Matrix<R, C>,
  ) {
    self.subtract_matrix(rhs);
  }
}
