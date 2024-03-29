// =============================================================================
//! - The Matrix and supporting structures
//!
//! # Metadata
//! - Copyright: &copy; 2022-2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Java updated: 1998-12-27
//! - Rust created: 2022-09-04
//! - Rust updated: 2023-07-26
//!
//! # History
//! - Adapted from the Java class com.croftsoft.core.math.Matrix
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.CroftSoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.CroftSoft.com/
//! [`David Wallace Croft`]: https://www.CroftSoft.com/people/david/
// =============================================================================

// -----------------------------------------------------------------------------
/// A newtype for functions that take an f64 argument in units of degrees
// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Degrees(pub f64);

// -----------------------------------------------------------------------------
/// The row and column indices of a Matrix, indexed from zero
// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Indices {
  pub row: usize,
  pub column: usize,
}

// -----------------------------------------------------------------------------
/// A mathematical matrix structure
// -----------------------------------------------------------------------------
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
  pub rows: [[f64; C]; R],
}

// -----------------------------------------------------------------------------
/// A newtype for functions that take an f64 argument in units of radians
// -----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Radians(pub f64);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RotationDegrees {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RotationRadians {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}
