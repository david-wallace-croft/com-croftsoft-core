// =============================================================================
//! - Financial calculations
//!
//! # Usage
//! - Since any time period can be used, "payment stream" equals "annuity"
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java created: 1999-08-15
//! - Java updated: 2001-10-10
//! - Rust created: 2022-07-30
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the Java class com.croftsoft.core.math.FinanceLib
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
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
/// Calculates the future value of a cash flow received today
///
/// # Example
/// ```
/// use com_croftsoft_core::math::finance_lib::FutureValue;
/// assert_eq!(
///   FutureValue {
///     cash_flow:     1.0,  // Investing $1 today
///     interest_rate: 0.12, // At 12% per year
///     time_periods:  6.0,  // For six years with interest compounded annually
///   }.calculate(),
///   1.973_822_685_184_001); // Will double your money (see the "Rule of 72")
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct FutureValue {
  /// Cash flow invested or received today
  pub cash_flow: f64,
  /// Interest, discount, or inflation rate (use 0.01 for 1%)
  pub interest_rate: f64,
  /// Number of periods from today when the value is evaluated
  pub time_periods: f64,
}

impl FutureValue {
  pub fn calculate(&self) -> f64 {
    let c = self.cash_flow;
    let r = self.interest_rate;
    let t = self.time_periods;
    c * (1.0 + r).powf(t)
  }
}

// -----------------------------------------------------------------------------
/// Calculates the future value of a payment stream such as an annuity
///
/// # Example
/// ```
/// use com_croftsoft_core::math::finance_lib::FutureValuePaymentStream;
/// assert_eq!(
///   FutureValuePaymentStream {
///     cash_income:   10_000.0, // Future payments of $10k per year
///     interest_rate: 0.10,     // With 10% interest annually on the payments
///     time_periods:  10.0,     // Paying each year for ten years
///   }.calculate(),
///   159_374.246_010_000_2);    // Will be worth ~$159k in the future
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct FutureValuePaymentStream {
  /// Periodic cash income payment starting one period from today
  pub cash_income: f64,
  /// Periodic interest earned on income (use 0.01 for 1%)
  pub interest_rate: f64,
  /// Number of periods of cash income
  pub time_periods: f64,
}

impl FutureValuePaymentStream {
  pub fn calculate(&self) -> f64 {
    let c = self.cash_income;
    let r = self.interest_rate;
    let t = self.time_periods;
    c * ((1.0 + r).powf(t) - 1.0) / r
  }
}

// -----------------------------------------------------------------------------
/// The calculated discount rate where the net present value is zero
///
/// # Examples
/// ```
/// use com_croftsoft_core::math::finance_lib::*;
/// static POSITIVE_FUTURE_CASH_FLOWS: [f64; 3] = [
///   -2.0,  // $2 paid out today
///   1.10,  // $1.10 received a year from today
///   1.21]; // and another $1.21 received two years from today
/// static NEGATIVE_FUTURE_CASH_FLOWS: [f64; 3] = [
///   2.0,    // $2 received today
///   -1.10,  // $1.10 paid out a year from today
///   -1.21]; // and another $1.21 paid out two years from today
/// static IRR_ESTIMATE: f64 = 0.01; // Initial IRR estimate is 1% per year
/// assert_eq!(
///   InternalRateOfReturn {
///     cash_flows: &POSITIVE_FUTURE_CASH_FLOWS,
///     irr_estimate: IRR_ESTIMATE,
///   }.calculate().unwrap(),
///   0.09999999999999998); // Calculated IRR ~10%
/// assert_eq!(
///   InternalRateOfReturn {
///     cash_flows: &POSITIVE_FUTURE_CASH_FLOWS,
///     irr_estimate: 0.0,  // Use zero when there is no initial estimate
///   }.calculate().unwrap(),
///   0.09999999999999998); // Calculated IRR ~10%
/// assert_eq!(
///   InternalRateOfReturn {
///     cash_flows: &NEGATIVE_FUTURE_CASH_FLOWS,
///     irr_estimate: IRR_ESTIMATE,
///   }.calculate().unwrap(),
///   0.09999999999999998); // Calculated IRR ~10%
/// assert_eq!(
///   InternalRateOfReturn {
///     cash_flows: &[-1.0, 1.0], // A dollar paid today is returned in a year
///     irr_estimate: IRR_ESTIMATE,
///   }.calculate().unwrap(),
///   -1.1053349683155043e-17); // Calculated IRR ~0%
/// assert_eq!(
///   InternalRateOfReturn {
///     cash_flows: &[0.0, -1.0, 1.1], // The first cash flow is in the future
///     irr_estimate: IRR_ESTIMATE,
///   }.calculate().unwrap(),
///   0.10000000000000009); // Calculated IRR ~10%
/// assert_eq!(
///   InternalRateOfReturn {
///     cash_flows: &[1.0, 0.0],
///     irr_estimate: IRR_ESTIMATE,
///   }.calculate().unwrap_err(),
///   InternalRateOfReturnError::CashFlowsAllNonNegative);
/// assert_eq!(
///   InternalRateOfReturn {
///     cash_flows: &[-1.0, 0.0],
///     irr_estimate: IRR_ESTIMATE,
///   }.calculate().unwrap_err(),
///   InternalRateOfReturnError::CashFlowsAllNonPositive);
/// assert_eq!(
///   InternalRateOfReturn {
///     cash_flows: &[0.0, 0.0], // All zero cash flows
///     irr_estimate: IRR_ESTIMATE,
///   }.calculate().unwrap_err(),
///   InternalRateOfReturnError::CashFlowsAllZero);
/// assert_eq!(
///   InternalRateOfReturn {
///     cash_flows: &[], // Zero length cash flows
///     irr_estimate: IRR_ESTIMATE,
///   }.calculate().unwrap_err(),
///   InternalRateOfReturnError::CashFlowsLengthLessThanTwo);
/// assert_eq!(
///   InternalRateOfReturn {
///     cash_flows: &[-1.0], // Single length cash flows
///     irr_estimate: IRR_ESTIMATE,
///   }.calculate().unwrap_err(),
///   InternalRateOfReturnError::CashFlowsLengthLessThanTwo);
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct InternalRateOfReturn<'a> {
  /// Array of cash flows received in the future, indexed from time zero
  pub cash_flows: &'a [f64],
  /// Initial estimate for the IRR (use 0.10 for 10%; use 0.0 for no estimate)
  pub irr_estimate: f64,
}

#[derive(Debug, Eq, PartialEq)]
pub enum InternalRateOfReturnError {
  CashFlowsAllNonNegative,
  CashFlowsAllNonPositive,
  CashFlowsAllZero,
  CashFlowsLengthLessThanTwo,
}

impl<'a> InternalRateOfReturn<'a> {
  pub fn calculate(&self) -> Result<f64, InternalRateOfReturnError> {
    if self.cash_flows.len() < 2 {
      return Err(InternalRateOfReturnError::CashFlowsLengthLessThanTwo);
    }
    let mut has_a_negative_cash_flow = false;
    let mut has_a_positive_cash_flow = false;
    for cash_flow in self.cash_flows {
      if *cash_flow < 0.0 {
        has_a_negative_cash_flow = true;
        if has_a_positive_cash_flow {
          break;
        }
      } else if *cash_flow > 0.0 {
        has_a_positive_cash_flow = true;
        if has_a_negative_cash_flow {
          break;
        }
      }
    }
    if has_a_negative_cash_flow {
      if !has_a_positive_cash_flow {
        return Err(InternalRateOfReturnError::CashFlowsAllNonPositive);
      }
    } else if has_a_positive_cash_flow {
      return Err(InternalRateOfReturnError::CashFlowsAllNonNegative);
    } else {
      return Err(InternalRateOfReturnError::CashFlowsAllZero);
    }
    let mut irr: f64 = self.irr_estimate;
    let mut delta_irr: f64 = -0.1 * irr;
    if delta_irr == 0.0 {
      delta_irr = 0.001;
    }
    let mut old_npv: f64 = core::f64::NAN;
    Ok(loop {
      let npv: f64 = NetPresentValue {
        cash_flows: self.cash_flows,
        discount_rate: irr,
      }
      .calculate();
      if npv == 0.0 {
        break irr;
      }
      if old_npv < 0.0 {
        if npv > 0.0 {
          delta_irr *= -0.9;
        } else if npv > old_npv {
          delta_irr *= 1.1;
        } else if npv < old_npv {
          delta_irr = -delta_irr;
        } else {
          // where npv == old_npv
          break irr;
        }
      } else if old_npv > 0.0 {
        if npv < 0.0 {
          delta_irr *= -0.9;
        } else if npv < old_npv {
          delta_irr *= 1.1;
        } else if npv > old_npv {
          delta_irr = -delta_irr;
        } else {
          // where npv == old_npv
          break irr;
        }
      }
      if delta_irr == 0.0 {
        break irr;
      }
      irr += delta_irr;
      old_npv = npv;
    })
  }
}

// -----------------------------------------------------------------------------
/// The discounted value of multiple cash flows received in the future
///
/// # Example
/// ```
/// use com_croftsoft_core::math::finance_lib::NetPresentValue;
/// assert_eq!(
///   NetPresentValue {
///     cash_flows:    &[1.0], // A dollar today
///     discount_rate: 0.10,   // At a discount rate of 10% per time period
///   }.calculate(),
///   1.0);                    // Is worth a dollar today
/// assert_eq!(
///   NetPresentValue {
///     cash_flows:    &[0.0, 1.0], // A dollar next year
///     discount_rate: 0.10,        // At a discount rate of 10% per year
///   }.calculate(),
///   0.9090909090909091);          // Is worth ~$0.91 today
/// assert_eq!(
///   NetPresentValue {
///     cash_flows:    &[0.0, 0.0, 1.0], // A dollar received in two years
///     discount_rate: 0.10,             // Discounted at 10% per year
///   }.calculate(),
///   0.8264462809917354);               // Is worth ~$0.83 today
/// assert_eq!(
///   NetPresentValue {
///     cash_flows:    &[1.0; 11], // $1 today plus $1 per year for 10 years
///     discount_rate: 0.10,       // At a discount rate of 10% per year
///   }.calculate(),
///   7.144567105704681);          // Is worth ~$7.14 today
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct NetPresentValue<'a> {
  // Cash flows received in the future indexed from time zero
  pub cash_flows: &'a [f64],
  /// The discount rate or cost of capital (use 0.01 for 1%)
  pub discount_rate: f64,
}

impl<'a> NetPresentValue<'a> {
  pub fn calculate(&self) -> f64 {
    self
      .cash_flows
      .iter()
      .enumerate()
      .fold(0.0, |sum, (index, cash_flow)| {
        sum + cash_flow / (1.0 + self.discount_rate).powf(index as f64)
      })
  }
}

// -----------------------------------------------------------------------------
/// Calculates the periodic investments required to accumulate a future value
///
/// # Examples
/// ```
/// use com_croftsoft_core::math::finance_lib::PeriodicSavingsNeeded;
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
///   future_value:  1_000_000.0,
///   interest_rate: 0.00,
///   time_periods:  10.0,
/// };
/// for index in 0..12 {
///   periodic_savings_needed.interest_rate = (index + 1) as f64 / 100.0;
///   calculated_values[index] = periodic_savings_needed.calculate();
/// }
/// assert_eq!(calculated_values[ 0], 95_582.076_551_171_35 ); // @  1%
/// assert_eq!(calculated_values[ 4], 79_504.574_965_456_62 ); // @  5%
/// assert_eq!(calculated_values[ 7], 69_029.488_697_075_34 ); // @  8%
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

// -----------------------------------------------------------------------------
/// The discounted value of a single cash flow received in the future
///
/// # Example
/// ```
/// use com_croftsoft_core::math::finance_lib::PresentValue;
/// assert_eq!(
///   PresentValue {
///     cash_flow:     1.0,  // A dollar in the future
///     discount_rate: 0.10, // With inflation at 10% per year
///     time_periods:  1.0,  // Received one year from now
///   }.calculate(),
///   0.9090909090909091);   // Will have the spending power of ~$0.91 today
/// assert_eq!(
///   PresentValue {
///     cash_flow:     1.0,  // A dollar in the future
///     discount_rate: 0.10, // If it could be invested today at 10% per year
///     time_periods:  2.0,  // Received two years from now
///   }.calculate(),
///   0.8264462809917354);   // Would be worth the same as ~$0.83 invested today
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct PresentValue {
  // Cash flow received in the future
  pub cash_flow: f64,
  /// The discount rate or inflation rate per time period (use 0.01 for 1%)
  pub discount_rate: f64,
  /// Number of time periods from today when the cash flow is received
  pub time_periods: f64,
}

impl PresentValue {
  pub fn calculate(&self) -> f64 {
    let c = self.cash_flow;
    let r = self.discount_rate;
    let t = self.time_periods;
    c / (1.0 + r).powf(t)
  }
}

// -----------------------------------------------------------------------------
/// The discounted value of varying periodic cash flows
///
/// # Example
/// ```
/// use com_croftsoft_core::math::finance_lib::PresentValueCashFlows;
/// assert_eq!(
///   PresentValueCashFlows {
///     cash_flows:    &[1.0], // A dollar received one year in the future
///     discount_rate: 0.10,   // With inflation at 10% per year
///   }.calculate(),
///   0.9090909090909091);     // Will have the spending power of ~$0.91 today
/// assert_eq!(
///   PresentValueCashFlows {
///     cash_flows:    &[0.0, 1.0], // A dollar received in two years
///     discount_rate: 0.10,        // With interest at 10% per year
///   }.calculate(),
///   0.8264462809917354);   // Would be worth the same as ~$0.83 invested today
/// assert_eq!(
///   PresentValueCashFlows {
///     cash_flows:    &[1.0, 2.0, 3.0], // $1, $2, and $3 over 3 years
///     discount_rate: 0.0,              // With no inflation
///   }.calculate(),
///   6.0);                              // Would be worth $6 today
/// assert_eq!(
///   PresentValueCashFlows {
///     cash_flows:    &[1.0, 2.0, 3.0], // $1, $2, and $3 over 3 years
///     discount_rate: 0.10,             // With inflation at 10% per year
///   }.calculate(),
///   4.8159278737791125);               // Would be worth ~$4.82 today
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct PresentValueCashFlows<'a> {
  // Cash flows received in the future starting one time period from now
  pub cash_flows: &'a [f64],
  /// The discount rate or inflation rate per time period (use 0.01 for 1%)
  pub discount_rate: f64,
}

impl<'a> PresentValueCashFlows<'a> {
  pub fn calculate(&self) -> f64 {
    self
      .cash_flows
      .iter()
      .enumerate()
      .fold(0.0, |sum, (index, cash_flow)| {
        sum
          + PresentValue {
            cash_flow: *cash_flow,
            discount_rate: self.discount_rate,
            time_periods: (index + 1) as f64,
          }
          .calculate()
      })
  }
}

// -----------------------------------------------------------------------------
/// Calculates the present value of a payment stream such as an annuity
///
/// # Example
/// ```
/// use com_croftsoft_core::math::finance_lib::PresentValuePaymentStream;
/// assert_eq!(
///   PresentValuePaymentStream {
///     cash_flow :     1.0,  // A dollar every year starting a year from today
///     inflation_rate: 0.10, // With inflation at 10% per year
///     time_periods:   10.0, // For ten years
///   }.calculate(),
///   6.144567105704685);     // Is the same as receiving ~$6.14 today
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub struct PresentValuePaymentStream {
  // Periodic cash income staring one time period from today
  pub cash_flow: f64,
  /// The inflation rate or interest rate per time period (use 0.01 for 1%)
  pub inflation_rate: f64,
  /// Number of time periods of cash income
  pub time_periods: f64,
}

impl PresentValuePaymentStream {
  pub fn calculate(&self) -> f64 {
    let c = self.cash_flow;
    let r = self.inflation_rate;
    let t = self.time_periods;
    c * (1.0 - 1.0 / (1.0 + r).powf(t)) / r
  }
}
