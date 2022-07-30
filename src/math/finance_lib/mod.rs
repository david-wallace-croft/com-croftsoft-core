// =============================================================================
//! - Financial calculations
//! - Rust version: 2022-07-30
//! - Rust since: 2022-07-30
//! - Adapted from the Java class com.croftsoft.core.math.FinanceLib
//! - <http://www.croftsoft.com/library/code/>
//! - Java version: 2001-10-10
//! - Java since: 1999-08-15
// =============================================================================

// -----------------------------------------------------------------------------
/// Calculates the annual savings necessary to accumulate a specified
/// value in the future
///
/// - `f` - Future value desired
/// - `r` - Annual interest
/// - `t` - Number of years of savings
// -----------------------------------------------------------------------------
pub fn annual_savings_needed(
  f: f64,
  r: f64,
  t: f64,
) -> f64 {
  f * r / ((1.0 + r).powf(t) - 1.0)
}
