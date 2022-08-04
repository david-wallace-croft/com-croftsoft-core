// =============================================================================
//! - Financial calculations
//! - Rust version: 2022-08-04
//! - Rust since: 2022-07-30
//! - Adapted from the Java class com.croftsoft.core.math.FinanceLib
//! - <http://www.croftsoft.com/library/code/>
//! - Java version: 2001-10-10
//! - Java since: 1999-08-15
// =============================================================================

// -----------------------------------------------------------------------------
/// Replaced by PeriodicSavingsNeeded
// -----------------------------------------------------------------------------
#[deprecated(since = "0.2.0", note = "Replaced by PeriodicSavingsNeeded")]
pub fn annual_savings_needed(
  f: f64,
  r: f64,
  t: f64,
) -> f64 {
  f * r / ((1.0 + r).powf(t) - 1.0)
}

// -----------------------------------------------------------------------------
/// Calculates the periodic investments required to accumulate a future value
///
/// # Examples
/// ```
/// use com_croftsoft_core::math::finance_lib::*;
/// assert_eq!(
///   PeriodicSavingsNeeded {
///     future_value:  1_000_000.0,   // To have a million dollars in the future
///     interest_rate: 0.12,          // At 12% interest compounded annually
///     time_periods:  10.0,          // Investing each year for ten years
///   }.calculate(),
///   56_984.164_159_844_026);        // Invest ~$57k per year
/// assert_eq!(
///   PeriodicSavingsNeeded {
///     future_value:  100_000_000.0, // To have a hundred million cents
///     interest_rate: 0.01,          // At 1% interest compounded monthly
///     time_periods:  120.0,         // Investing each month for 120 months
///   }.calculate(),
///   434_709.484_025_873_1); // Invest ~435k cents per month (~$52k per year)
/// let mut calculated_values = [0.0; 12];
/// let mut periodic_savings_needed = PeriodicSavingsNeeded {
///   future_value: 1_000_000.0,
///   interest_rate: 0.00,
///   time_periods: 10.0,
/// };
/// for index in 0..12 {
///   periodic_savings_needed.interest_rate = (index + 1) as f64 / 100.0;
///   calculated_values[index] = periodic_savings_needed.calculate();
/// }
/// assert_eq!(calculated_values[ 0], 95_582.076_551_171_35 ); // @ 1%
/// assert_eq!(calculated_values[ 4], 79_504.574_965_456_62 ); // @ 5%
/// assert_eq!(calculated_values[ 7], 69_029.488_697_075_34 ); // @ 8%
/// assert_eq!(calculated_values[11], 56_984.164_159_844_026); // @ 12%
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct PeriodicSavingsNeeded {
  /// Future value desired
  pub future_value: f64,
  /// Periodic interest rate (use 0.01 for 1%)
  pub interest_rate: f64,
  /// Number of time periods of investment
  pub time_periods: f64,
}

impl PeriodicSavingsNeeded {
  pub fn calculate(&self) -> f64 {
    let f = self.future_value;
    let r = self.interest_rate;
    let t = self.time_periods;
    f * r / ((1.0 + r).powf(t) - 1.0)
  }
}
