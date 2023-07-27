// =============================================================================
//! - A collection of math constants
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java updated: 2002-01-27
//! - Rust created: 2022-07-30
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the Java class com.croftsoft.core.math.MathConstants
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
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
