// =============================================================================
//! - A collection of math functions
//!
//! # Metadata
//! - Copyright: &copy; 1998 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-08-27
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
/// # Alternative
/// <https://doc.rust-lang.org/std/primitive.f64.html#method.clamp>
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
///     maximum:  f64::NAN,
///     minimum: -1.0,
///     value:    0.0,
///   }.calculate().unwrap_err(),
///   ClipError::MaximumIsNotANumber);
/// assert_eq!(
///   Clip {
///     maximum:  f64::INFINITY,
///     minimum: -1.0,
///     value:    0.0,
///   }.calculate().unwrap_err(),
///   ClipError::MaximumIsInfinite(f64::INFINITY));
/// assert_eq!(
///   Clip {
///     maximum:  f64::NEG_INFINITY,
///     minimum: -1.0,
///     value:    0.0,
///   }.calculate().unwrap_err(),
///   ClipError::MaximumIsInfinite(f64::NEG_INFINITY));
/// assert_eq!(
///   Clip {
///     maximum: -1.0,
///     minimum:  1.0,
///     value:    0.0,
///   }.calculate().unwrap_err(),
///   ClipError::MinimumIsGreaterThanMaximum);
/// assert_eq!(
///   Clip {
///     maximum:  1.0,
///     minimum:  f64::INFINITY,
///     value:    0.0,
///   }.calculate().unwrap_err(),
///   ClipError::MinimumIsInfinite(f64::INFINITY));
/// assert_eq!(
///   Clip {
///     maximum:  1.0,
///     minimum:  f64::NEG_INFINITY,
///     value:    0.0,
///   }.calculate().unwrap_err(),
///   ClipError::MinimumIsInfinite(f64::NEG_INFINITY));
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
///     value:    f64::INFINITY,
///   }.calculate().unwrap_err(),
///   ClipError::ValueIsInfinite(f64::INFINITY));
/// assert_eq!(
///   Clip {
///     maximum:  1.0,
///     minimum: -1.0,
///     value:    f64::NEG_INFINITY,
///   }.calculate().unwrap_err(),
///   ClipError::ValueIsInfinite(f64::NEG_INFINITY));
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

#[derive(Debug, PartialEq)]
pub enum ClipError {
  MaximumIsInfinite(f64),
  MaximumIsNotANumber,
  MinimumIsGreaterThanMaximum,
  MinimumIsInfinite(f64),
  MinimumIsNotANumber,
  ValueIsInfinite(f64),
  ValueIsNotANumber,
}

impl Clip {
  pub fn calculate(&self) -> Result<f64, ClipError> {
    let max = self.maximum;
    let min = self.minimum;
    let val = self.value;
    if max.is_infinite() {
      return Err(ClipError::MaximumIsInfinite(max));
    }
    if min.is_infinite() {
      return Err(ClipError::MinimumIsInfinite(min));
    }
    if val.is_infinite() {
      return Err(ClipError::ValueIsInfinite(val));
    }
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
      return Err(ClipError::MinimumIsGreaterThanMaximum);
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

// -----------------------------------------------------------------------------
/// Cumulative Distribution Function (CDF)
///
/// # Links
/// <https://en.wikipedia.org/wiki/Cumulative_distribution_function>
// -----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct CumulativeDistributionFunction {
  pub x: f64,
  pub lambda: f64,
}

impl CumulativeDistributionFunction {
  pub fn calculate(&self) -> f64 {
    if self.x <= 0.0 {
      return 0.0;
    }
    1.0 - (-self.lambda * self.x).exp()
  }
}

// -----------------------------------------------------------------------------
/// Wraps the value to [minimum, minimum + range)
///
/// # Examples
/// ```
/// use com_croftsoft_core::math::math_lib::*;
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    360.0,
///     value:   -190.0,
///   }.calculate().unwrap(),
///   170.0);
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    360.0,
///     value:    190.0,
///   }.calculate().unwrap(),
///   -170.0);
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    360.0,
///     value:    180.0,
///   }.calculate().unwrap(),
///   -180.0);
/// assert_eq!(
///   Wrap {
///     minimum:  f64::INFINITY,
///     range:    360.0,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::MinimumIsInfinite(f64::INFINITY));
/// assert_eq!(
///   Wrap {
///     minimum:  f64::NEG_INFINITY,
///     range:    360.0,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::MinimumIsInfinite(f64::NEG_INFINITY));
/// assert_eq!(
///   Wrap {
///     minimum:  f64::NAN,
///     range:    360.0,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::MinimumIsNotANumber);
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    f64::INFINITY,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::RangeIsInfinite(f64::INFINITY));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    f64::NEG_INFINITY,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::RangeIsInfinite(f64::NEG_INFINITY));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:   -360.0,
///     value:    180.0,
///   }.calculate().unwrap_err(),
///   WrapError::RangeIsNonPositive(-360.0));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    f64::NAN,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::RangeIsNotANumber);
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    360.0,
///     value:    f64::INFINITY,
///   }.calculate().unwrap_err(),
///   WrapError::ValueIsInfinite(f64::INFINITY));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    360.0,
///     value:    f64::NEG_INFINITY,
///   }.calculate().unwrap_err(),
///   WrapError::ValueIsInfinite(f64::NEG_INFINITY));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    360.0,
///     value:    f64::NAN,
///   }.calculate().unwrap_err(),
///   WrapError::ValueIsNotANumber);
/// ```
// -----------------------------------------------------------------------------
pub struct Wrap {
  pub minimum: f64,
  pub range: f64,
  pub value: f64,
}

#[derive(Debug, PartialEq)]
pub enum WrapError {
  MinimumIsInfinite(f64),
  MinimumIsNotANumber,
  RangeIsInfinite(f64),
  RangeIsNonPositive(f64),
  RangeIsNotANumber,
  ValueIsInfinite(f64),
  ValueIsNotANumber,
}

impl Wrap {
  pub fn calculate(&self) -> Result<f64, WrapError> {
    let min = self.minimum;
    let rng = self.range;
    let val = self.value;
    if min.is_infinite() {
      return Err(WrapError::MinimumIsInfinite(min));
    }
    if rng.is_infinite() {
      return Err(WrapError::RangeIsInfinite(rng));
    }
    if val.is_infinite() {
      return Err(WrapError::ValueIsInfinite(val));
    }
    if min.is_nan() {
      return Err(WrapError::MinimumIsNotANumber);
    }
    if rng.is_nan() {
      return Err(WrapError::RangeIsNotANumber);
    }
    if val.is_nan() {
      return Err(WrapError::ValueIsNotANumber);
    }
    if rng <= 0.0 {
      return Err(WrapError::RangeIsNonPositive(rng));
    }
    let max = min + rng;
    if val < min {
      Ok(val + ((min - val) / rng).ceil() * rng)
    } else if val < max {
      Ok(val)
    } else {
      Ok(val - (1.0 + ((val - max) / rng).floor()) * rng)
    }
  }
}
