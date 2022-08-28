// =============================================================================
//! - A collection of math functions
//!
//! # Metadata
//! - Copyright: &copy; 1998 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-08-28
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
///     minimum:  f64::MAX,
///     range:    360.0,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::FloatResolution(
///     WrapErrorFloatResolution::DeltaIsNegativeMinimum));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    360.0,
///     value:    f64::MAX,
///   }.calculate().unwrap_err(),
///   WrapError::FloatResolution(
///     WrapErrorFloatResolution::DeltaIsValue));
/// assert_eq!(
///   Wrap {
///     minimum:  f64::INFINITY,
///     range:    360.0,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::InvalidArgument(
///     WrapErrorInvalidArgument::MinimumIsInfinite(f64::INFINITY)));
/// assert_eq!(
///   Wrap {
///     minimum:  f64::NEG_INFINITY,
///     range:    360.0,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::InvalidArgument(
///     WrapErrorInvalidArgument::MinimumIsInfinite(f64::NEG_INFINITY)));
/// assert_eq!(
///   Wrap {
///     minimum:  f64::NAN,
///     range:    360.0,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::InvalidArgument(
///     WrapErrorInvalidArgument::MinimumIsNotANumber));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    f64::INFINITY,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::InvalidArgument(
///     WrapErrorInvalidArgument::RangeIsInfinite(f64::INFINITY)));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    f64::NEG_INFINITY,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::InvalidArgument(
///     WrapErrorInvalidArgument::RangeIsInfinite(f64::NEG_INFINITY)));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:   -360.0,
///     value:    180.0,
///   }.calculate().unwrap_err(),
///   WrapError::InvalidArgument(
///     WrapErrorInvalidArgument::RangeIsNonPositive(-360.0)));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    f64::NAN,
///     value:    190.0,
///   }.calculate().unwrap_err(),
///   WrapError::InvalidArgument(
///     WrapErrorInvalidArgument::RangeIsNotANumber));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    360.0,
///     value:    f64::INFINITY,
///   }.calculate().unwrap_err(),
///   WrapError::InvalidArgument(
///     WrapErrorInvalidArgument::ValueIsInfinite(f64::INFINITY)));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    360.0,
///     value:    f64::NEG_INFINITY,
///   }.calculate().unwrap_err(),
///   WrapError::InvalidArgument(
///     WrapErrorInvalidArgument::ValueIsInfinite(f64::NEG_INFINITY)));
/// assert_eq!(
///   Wrap {
///     minimum: -180.0,
///     range:    360.0,
///     value:    f64::NAN,
///   }.calculate().unwrap_err(),
///   WrapError::InvalidArgument(
///     WrapErrorInvalidArgument::ValueIsNotANumber));
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct Wrap {
  pub minimum: f64,
  pub range: f64,
  pub value: f64,
}

#[derive(Debug, PartialEq)]
pub enum WrapError {
  FloatResolution(WrapErrorFloatResolution),
  InvalidArgument(WrapErrorInvalidArgument),
}

#[derive(Debug, PartialEq)]
pub enum WrapErrorFloatResolution {
  CalculatedIsLessThanMinimum(f64),
  CalculatedIsNotLessThanMinimumPlusRange(f64),
  CyclesIsZero,
  DeltaIsNegativeMinimum,
  DeltaIsValue,
  OffsetIsZero,
}

#[derive(Debug, PartialEq)]
pub enum WrapErrorInvalidArgument {
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
      return Err(WrapError::InvalidArgument(
        WrapErrorInvalidArgument::MinimumIsInfinite(min),
      ));
    }
    if rng.is_infinite() {
      return Err(WrapError::InvalidArgument(
        WrapErrorInvalidArgument::RangeIsInfinite(rng),
      ));
    }
    if val.is_infinite() {
      return Err(WrapError::InvalidArgument(
        WrapErrorInvalidArgument::ValueIsInfinite(val),
      ));
    }
    if min.is_nan() {
      return Err(WrapError::InvalidArgument(
        WrapErrorInvalidArgument::MinimumIsNotANumber,
      ));
    }
    if rng.is_nan() {
      return Err(WrapError::InvalidArgument(
        WrapErrorInvalidArgument::RangeIsNotANumber,
      ));
    }
    if val.is_nan() {
      return Err(WrapError::InvalidArgument(
        WrapErrorInvalidArgument::ValueIsNotANumber,
      ));
    }
    if rng <= 0.0 {
      return Err(WrapError::InvalidArgument(
        WrapErrorInvalidArgument::RangeIsNonPositive(rng),
      ));
    }
    let max = min + rng;
    if min <= val && val < max {
      return Ok(val);
    }
    let delta = val - min;
    if delta == -min {
      return Err(WrapError::FloatResolution(
        WrapErrorFloatResolution::DeltaIsNegativeMinimum,
      ));
    }
    if delta == val {
      return Err(WrapError::FloatResolution(
        WrapErrorFloatResolution::DeltaIsValue,
      ));
    }
    let cycles = (delta / rng).floor();
    if cycles == 0.0 {
      return Err(WrapError::FloatResolution(
        WrapErrorFloatResolution::CyclesIsZero,
      ));
    }
    let offset = cycles * rng;
    if offset == 0.0 {
      return Err(WrapError::FloatResolution(
        WrapErrorFloatResolution::OffsetIsZero,
      ));
    }
    let calculated = val - offset;
    if calculated < min {
      return Err(WrapError::FloatResolution(
        WrapErrorFloatResolution::CalculatedIsLessThanMinimum(calculated),
      ));
    }
    if calculated >= max {
      return Err(WrapError::FloatResolution(
        WrapErrorFloatResolution::CalculatedIsNotLessThanMinimumPlusRange(
          calculated,
        ),
      ));
    }
    Ok(calculated)
  }
}
