// =============================================================================
//! - Unit tests for the Matrix operators
//!
//! # Metadata
//! - Copyright: &copy; 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Version: 2022-09-29
//! - Since: 2022-09-04
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
use super::super::structures::*;

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
fn test_div() {
  let matrix_2 = Matrix::<1, 1>::new(2.0);
  // dividend and f64
  assert_eq!(Matrix::<1, 1>::new(6.0) / 3.0, matrix_2);
  // &divident and f64
  assert_eq!(&Matrix::<1, 1>::new(6.0) / 3.0, matrix_2);
}

#[test]
fn test_div_assign() {
  let matrix_2 = Matrix::<1, 2>::new(2.0);
  // dividend and f64
  let mut dividend = Matrix::<1, 2>::new(6.0);
  dividend /= 3.0;
  assert_eq!(dividend, matrix_2);
  // &dividend and f64
  let mut dividend = &mut Matrix::<1, 2>::new(6.0);
  dividend /= 3.0;
  assert_eq!(dividend, &matrix_2);
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
fn test_neg() {
  // matrix
  assert_eq!(-Matrix::<1, 1>::new(1.0), Matrix::new(-1.0));
  // &matrix
  assert_eq!(-&Matrix::<1, 1>::new(1.0), Matrix::new(-1.0));
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
