// =============================================================================
//! - Financial calculations
//! - Rust version: 2022-08-19
//! - Rust since: 2022-07-30
//! - Adapted from the Java class com.croftsoft.core.math.FinanceLib
//! - <https://www.croftsoft.com/library/code/>
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
/// Calculates the future value of a cash flow received today.
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
/// Calculates the future value of a payment stream such as an annuity.
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
/// The calculated discount rate where the net present value is zero.
///
/// # Example
/// ```
/// use com_croftsoft_core::math::finance_lib::InternalRateOfReturn;
/// assert_eq!(
///   InternalRateOfReturn {
///     cash_flows:   &vec![-2.0, 1.1, 1.2], // -$2 now, $1.1 in 1y, $1.2 in 2y
///     irr_estimate: 0.10,                  // Estimated IRR of 10%
///   }.calculate(),
///   0.09696411113867977);                  // Calculated IRR ~9.7%
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct InternalRateOfReturn<'a> {
  /// Array of cash flows received in the future, indexed from time zero
  pub cash_flows: &'a Vec<f64>,
  /// The initial estimated value for the IRR (use 0.10 for 10%)
  pub irr_estimate: f64,
}

impl<'a> InternalRateOfReturn<'a> {
  pub fn calculate(&self) -> f64 {
    let mut irr: f64 = self.irr_estimate;
    let mut delta = -irr * 0.1;
    let mut old_npv = 0.0;
    loop {
      let npv: f64 = NetPresentValue {
        cash_flows: self.cash_flows,
        discount_rate: irr,
      }
      .calculate();
      if npv == 0.0 {
        return irr;
      }
      if old_npv < 0.0 {
        if npv > 0.0 {
          delta *= -0.9;
        } else if npv > old_npv {
          delta *= 1.1;
        } else if npv < old_npv {
          delta = -delta;
        } else {
          delta = 0.0;
        }
      } else if old_npv > 0.0 {
        if npv < 0.0 {
          delta *= -0.9;
        } else if npv < old_npv {
          delta *= 1.1;
        } else if npv > old_npv {
          delta = -delta;
        } else {
          delta = 0.0;
        }
      }
      if delta == 0.0 {
        return irr;
      }
      irr += delta;
      old_npv = npv;
    }
  }
}

// -----------------------------------------------------------------------------
/// The discounted value of multiple cash flows received in the future.
///
/// # Example
/// ```
/// use com_croftsoft_core::math::finance_lib::NetPresentValue;
/// assert_eq!(
///   NetPresentValue {
///     cash_flows:    &vec![1.0], // A dollar today
///     discount_rate: 0.10,       // At a discount rate of 10% per time period
///   }.calculate(),
///   1.0);                        // Is worth a dollar today
/// assert_eq!(
///   NetPresentValue {
///     cash_flows:    &vec![0.0, 1.0], // A dollar next year
///     discount_rate: 0.10,            // At a discount rate of 10% per year
///   }.calculate(),
///   0.9090909090909091);              // Is worth ~$0.91 today
/// assert_eq!(
///   NetPresentValue {
///     cash_flows:    &vec![0.0, 0.0, 1.0], // A dollar received in two years
///     discount_rate: 0.10,                 // Discounted at 10% per year
///   }.calculate(),
///   0.8264462809917354);                   // Is worth ~$0.83 today
/// assert_eq!(
///   NetPresentValue {
///     cash_flows:    &vec![1.0; 11], // $1 today plus $1 per year for 10 years
///     discount_rate: 0.10,           // At a discount rate of 10% per year
///   }.calculate(),
///   7.144567105704681);              // Is worth ~$7.14 today
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct NetPresentValue<'a> {
  // Cash flows received in the future indexed from time zero
  pub cash_flows: &'a Vec<f64>,
  /// The discount rate or cost of capital (use 0.01 for 1%)
  pub discount_rate: f64,
}

impl<'a> NetPresentValue<'a> {
  pub fn calculate(&self) -> f64 {
    let mut net_present_value = 0.0;
    for (index, cash_flow) in self.cash_flows.iter().enumerate() {
      net_present_value +=
        cash_flow / (1.0 + self.discount_rate).powf(index as f64);
    }
    net_present_value
  }
}

// -----------------------------------------------------------------------------
/// Calculates the periodic investments required to accumulate a future value.
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
/// The discounted value of a single cash flow received in the future.
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
/// The discounted value of varying periodic cash flows.
///
/// # Example
/// ```
/// use com_croftsoft_core::math::finance_lib::PresentValueCashFlows;
/// assert_eq!(
///   PresentValueCashFlows {
///     cash_flows:    &vec![1.0],  // A dollar received one year in the future
///     discount_rate: 0.10,        // With inflation at 10% per year
///   }.calculate(),
///   0.9090909090909091);   // Will have the spending power of ~$0.91 today
/// assert_eq!(
///   PresentValueCashFlows {
///     cash_flows:    &vec![0.0, 1.0],  // A dollar received in two years
///     discount_rate: 0.10,             // With inflation at 10% per year
///   }.calculate(),
///   0.8264462809917354);   // Would be worth the same as ~$0.83 invested today
/// assert_eq!(
///   PresentValueCashFlows {
///     cash_flows:    &vec![1.0, 2.0, 3.0], // $1, $2, and $3 over 3 years
///     discount_rate: 0.0,                  // With no inflation
///   }.calculate(),
///   6.0);                                  // Would be worth $6 today
/// assert_eq!(
///   PresentValueCashFlows {
///     cash_flows:    &vec![1.0, 2.0, 3.0], // $1, $2, and $3 over 3 years
///     discount_rate: 0.10,                 // With inflation at 10% per year
///   }.calculate(),
///   4.8159278737791125);                   // Would be worth ~$4.82 today
/// ```
// -----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct PresentValueCashFlows<'a> {
  // Cash flows received in the future starting one time period from now
  pub cash_flows: &'a Vec<f64>,
  /// The discount rate or inflation rate per time period (use 0.01 for 1%)
  pub discount_rate: f64,
}

impl<'a> PresentValueCashFlows<'a> {
  pub fn calculate(&self) -> f64 {
    self.cash_flows.iter().enumerate().fold(0.0, |sum, (index, cash_flow)| {
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
