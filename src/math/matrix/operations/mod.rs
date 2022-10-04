// =============================================================================
//! - Overloaded operators for the structure Matrix
//! - Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign
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
use std::ops::{
  Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

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

// Operator Div ----------------------------------------------------------------

impl<const R: usize, const C: usize> Div<f64> for Matrix<R, C> {
  type Output = Self;

  fn div(
    self,
    rhs: f64,
  ) -> Self::Output {
    Self::divide_matrix_by_scalar(&self, rhs)
  }
}

impl<const R: usize, const C: usize> Div<f64> for &Matrix<R, C> {
  type Output = Matrix<R, C>;

  fn div(
    self,
    rhs: f64,
  ) -> Self::Output {
    Matrix::divide_matrix_by_scalar(self, rhs)
  }
}

// Operator DivAssign ----------------------------------------------------------

impl<const R: usize, const C: usize> DivAssign<f64> for Matrix<R, C> {
  fn div_assign(
    &mut self,
    rhs: f64,
  ) {
    self.divide_by_scalar(rhs);
  }
}

impl<const R: usize, const C: usize> DivAssign<f64> for &mut Matrix<R, C> {
  fn div_assign(
    &mut self,
    rhs: f64,
  ) {
    self.divide_by_scalar(rhs);
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
