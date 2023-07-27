// =============================================================================
//! - A collection of math functions
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 1998-12-27
//! - Java updated: 2008-08-09
//! - Rust created: 2022-08-24
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the Java class com.croftsoft.core.math.MathLib
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
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
#[derive(Clone, Debug, PartialEq)]
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
    let max: f64 = self.maximum;
    let min: f64 = self.minimum;
    let val: f64 = self.value;
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
    Ok(val.clamp(min, max))
  }
}

// -----------------------------------------------------------------------------
/// Cumulative Distribution Function (CDF)
///
/// # Links
/// <https://en.wikipedia.org/wiki/Cumulative_distribution_function>
// -----------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
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
/// Coordinates specified as angle and radius from the origin
// -----------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct PolarCoordinates {
  pub angle: f64,
  pub radius: f64,
}

impl PolarCoordinates {
  // ---------------------------------------------------------------------------
  /// Converts from polar to rectangular coordinates
  ///
  /// # Examples
  /// ```
  /// use com_croftsoft_core::math::math_lib::*;
  /// assert_eq!(
  ///   PolarCoordinates {
  ///     angle: 0.0,
  ///     radius: 1.0,
  ///   }.to_rectangular_coordinates(),
  ///   RectangularCoordinates {
  ///     x: 1.0,
  ///     y: 0.0,
  ///   });
  /// assert_eq!(
  ///   PolarCoordinates {
  ///     angle: core::f64::consts::FRAC_PI_2,
  ///     radius: 1.0,
  ///   }.to_rectangular_coordinates(),
  ///   RectangularCoordinates {
  ///     x: 6.123233995736766e-17,
  ///     y: 1.0,
  ///   });
  /// assert_eq!(
  ///   PolarCoordinates {
  ///     angle: core::f64::consts::PI,
  ///     radius: 1.0,
  ///   }.to_rectangular_coordinates(),
  ///   RectangularCoordinates {
  ///     x: -1.0,
  ///     y: 1.2246467991473532e-16,
  ///   });
  /// assert_eq!(
  ///   PolarCoordinates {
  ///     angle: 3.0 * core::f64::consts::FRAC_PI_2,
  ///     radius: 2.0,
  ///   }.to_rectangular_coordinates(),
  ///   RectangularCoordinates {
  ///     x: -3.6739403974420594e-16,
  ///     y: -2.0,
  ///   });
  /// ```
  // ---------------------------------------------------------------------------
  pub fn to_rectangular_coordinates(&self) -> RectangularCoordinates {
    let angle = self.angle;
    let radius = self.radius;
    RectangularCoordinates {
      x: radius * angle.cos(),
      y: radius * angle.sin(),
    }
  }
}

// -----------------------------------------------------------------------------
/// Cartesian (x, y) coordinates
// -----------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct RectangularCoordinates {
  pub x: f64,
  pub y: f64,
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
#[derive(Clone, Debug, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
pub enum FactorError {
  ArgumentIsZeroOrOne(u64),
}

// -----------------------------------------------------------------------------
/// Factors a number into its primes
/// ```
/// use com_croftsoft_core::math::math_lib::*;
/// assert_eq!(factor(0).unwrap_err(), FactorError::ArgumentIsZeroOrOne(0));
/// assert_eq!(factor(1).unwrap_err(), FactorError::ArgumentIsZeroOrOne(1));
/// assert_eq!(factor(2).unwrap(), vec!(2));
/// assert_eq!(factor(3).unwrap(), vec!(3));
/// assert_eq!(factor(4).unwrap(), vec!(2, 2));
/// assert_eq!(factor(5).unwrap(), vec!(5));
/// assert_eq!(factor(6).unwrap(), vec!(2, 3));
/// assert_eq!(factor(7).unwrap(), vec!(7));
/// assert_eq!(factor(8).unwrap(), vec!(2, 2, 2));
/// assert_eq!(factor(9).unwrap(), vec!(3, 3));
/// ```
// -----------------------------------------------------------------------------
pub fn factor(n: u64) -> Result<Vec<u64>, FactorError> {
  if n < 2 {
    return Err(FactorError::ArgumentIsZeroOrOne(n));
  }
  let mut prime_vec = Vec::new();
  let mut dividend = n;
  let mut divisor = 2;
  loop {
    if dividend % divisor == 0 {
      prime_vec.push(divisor);
      dividend /= divisor;
      if dividend == 1 {
        break;
      }
    } else {
      divisor += 1;
    }
  }
  Ok(prime_vec)
}

#[derive(Debug, Eq, PartialEq)]
pub enum GreatestCommonFactorError {
  ArgumentIsZero,
}

// -----------------------------------------------------------------------------
/// Computes the greatest common factor for two positive integers
///
/// ```
/// use com_croftsoft_core::math::math_lib::*;
/// assert_eq!(
///   greatest_common_factor(0, 1).unwrap_err(),
///   GreatestCommonFactorError::ArgumentIsZero);
/// assert_eq!(greatest_common_factor(1, 2).unwrap(), 1);
/// assert_eq!(greatest_common_factor(2, 3).unwrap(), 1);
/// assert_eq!(greatest_common_factor(2, 4).unwrap(), 2);
/// assert_eq!(greatest_common_factor(3, 6).unwrap(), 3);
/// ```
// -----------------------------------------------------------------------------
pub fn greatest_common_factor(
  n0: u64,
  n1: u64,
) -> Result<u64, GreatestCommonFactorError> {
  if n0 == 0 || n1 == 0 {
    return Err(GreatestCommonFactorError::ArgumentIsZero);
  }
  if n0 == 1 || n1 == 1 {
    return Ok(1);
  }
  let mut gcf: u64 = 1;
  let factor_vec_0 = factor(n0).unwrap();
  let mut factor_vec_1 = factor(n1).unwrap();
  for (index, factor_0) in factor_vec_0.iter().enumerate() {
    if factor_vec_1.contains(factor_0) {
      gcf *= factor_0;
      factor_vec_1.remove(index);
    }
  }
  Ok(gcf)
}

// -----------------------------------------------------------------------------
/// The sigmoid or logistic function
///
/// # Examples
/// ```
/// use com_croftsoft_core::math::math_lib::sigmoid;
/// assert_eq!(
///   sigmoid(f64::NEG_INFINITY),
///   0.0);
/// assert_eq!(
///   sigmoid(-1.0),
///   0.2689414213699951);
/// assert_eq!(
///   sigmoid(0.0),
///   0.5);
/// assert_eq!(
///   sigmoid(1.0),
///   0.7310585786300049);
/// assert_eq!(
///   sigmoid(f64::INFINITY),
///   1.0);
/// ```
// -----------------------------------------------------------------------------
pub fn sigmoid(a: f64) -> f64 {
  1.0 / (1.0 + (-a).exp())
}

// -----------------------------------------------------------------------------
/// The derivative of the sigmoid function with respect to the argument
///
/// # Examples
/// ```
/// use com_croftsoft_core::math::math_lib::sigmoid_derivative;
/// assert_eq!(
///   sigmoid_derivative(f64::NEG_INFINITY),
///   0.0);
/// assert_eq!(
///   sigmoid_derivative(-1.0),
///   0.19661193324148185);
/// assert_eq!(
///   sigmoid_derivative(0.0),
///   0.25);
/// assert_eq!(
///   sigmoid_derivative(1.0),
///   0.19661193324148185);
/// assert_eq!(
///   sigmoid_derivative(f64::INFINITY),
///   0.0);
/// ```
// -----------------------------------------------------------------------------
pub fn sigmoid_derivative(a: f64) -> f64 {
  let s = sigmoid(a);
  s * (1.0 - s)
}
