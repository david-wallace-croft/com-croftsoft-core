// =============================================================================
//! - A collection of math constants
//! - Rust version: 2022-07-30
//! - Rust since: 2022-07-30
//! - Adapted from the Java class com.croftsoft.core.math.MathConstants
//! - <http://www.croftsoft.com/library/code/>
//! - Java version: 2002-01-27
// =============================================================================

use core::f64::consts::TAU;

pub const DEGREES_PER_RADIAN: f64 = 360.0 / TAU;
pub const GOLDEN_RATIO: f64 = 1.618_033_988_749_895;
pub const SECONDS_PER_NANOSECOND: f64 = 0.000_000_001;
pub const MILLISECONDS_PER_DAY: u64 = 1_000 * 60 * 60 * 24;
pub const MILLISECONDS_PER_SECOND: u64 = 1_000;
pub const NANOSECONDS_PER_MICROSECOND: u64 = 1_000;
pub const NANOSECONDS_PER_MILLISECOND: u64 = 1_000_000;
pub const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;
