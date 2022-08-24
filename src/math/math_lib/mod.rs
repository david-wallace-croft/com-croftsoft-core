// =============================================================================
//! - A collection of math functions
//!
//! # Metadata
//! - Copyright: &copy; 1998 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-08-24
//! - Rust since: 2022-08-24
//! - Java version: 2008-08-09
//! - Java since: 1998-12-27
//!
//! # History
//! - Adapted from the Java class com.croftsoft.core.math.MathLib
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

// -----------------------------------------------------------------------------
/// Clips the value to a minimum and maximum range
///
/// # Examples
/// ```
/// use com_croftsoft_core::math::math_lib::*;
/// assert_eq!(
///   Clip {
///     maximum:  1.0,
///     minimum: -1.0,
///     value:    0.0,
///   }.calculate().unwrap(),
///   0.0);
/// assert_eq!(
///   Clip {
///     maximum:  1.0,
///     minimum: -1.0,
///     value:   -2.0,
///   }.calculate().unwrap(),
///   -1.0);
/// assert_eq!(
///   Clip {
///     maximum:  1.0,
///     minimum: -1.0,
///     value:    2.0,
///   }.calculate().unwrap(),
///   1.0);
/// assert_eq!(
///   Clip {
///     maximum: -1.0,
///     minimum:  1.0,
///     value:    0.0,
///   }.calculate().unwrap_err(),
///   ClipError::MinimumGreaterThanMaximum);
/// assert_eq!(
///   Clip {
///     maximum:  f64::NAN,
///     minimum: -1.0,
///     value:    0.0,
///   }.calculate().unwrap_err(),
///   ClipError::MaximumIsNotANumber);
/// assert_eq!(
///   Clip {
///     maximum:  1.0,
///     minimum:  f64::NAN,
///     value:    0.0,
///   }.calculate().unwrap_err(),
///   ClipError::MinimumIsNotANumber);
/// assert_eq!(
///   Clip {
///     maximum:  1.0,
///     minimum: -1.0,
///     value:    f64::NAN,
///   }.calculate().unwrap_err(),
///   ClipError::ValueIsNotANumber);
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct Clip {
  pub maximum: f64,
  pub minimum: f64,
  pub value: f64,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ClipError {
  MaximumIsNotANumber,
  MinimumGreaterThanMaximum,
  MinimumIsNotANumber,
  ValueIsNotANumber,
}

impl Clip {
  pub fn calculate(&self) -> Result<f64, ClipError> {
    let max = self.maximum;
    let min = self.minimum;
    let val = self.value;
    if max.is_nan() {
      return Err(ClipError::MaximumIsNotANumber);
    }
    if min.is_nan() {
      return Err(ClipError::MinimumIsNotANumber);
    }
    if val.is_nan() {
      return Err(ClipError::ValueIsNotANumber);
    }
    if min > max {
      return Err(ClipError::MinimumGreaterThanMaximum);
    }
    Ok(if val < min {
      min
    } else if val > max {
      max
    } else {
      val
    })
  }
}
