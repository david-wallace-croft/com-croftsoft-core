// =============================================================================
//! - Trait implementations for the structure Matrix
//!
//! # Metadata
//! - Copyright: &copy; 1998 - 2022 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Rust version: 2022-10-04
//! - Rust since: 2022-09-04
//! - Java version: 1998-12-27
//!
//! # History
//! - Adapted from the Java class com.croftsoft.core.math.Matrix
//!   - In the Java-based [`CroftSoft Core Library`]
//!
//! [`CroftSoft Core Library`]: https://www.croftsoft.com/library/code/
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

#[cfg(test)]
mod test;

use super::structures::*;

// Default ---------------------------------------------------------------------

impl<const R: usize, const C: usize> Default for Matrix<R, C> {
  // ---------------------------------------------------------------------------
  /// Makes a new Matrix of all zero entries
  // ---------------------------------------------------------------------------
  fn default() -> Self {
    Self {
      rows: [[0.0; C]; R],
    }
  }
}

// From ------------------------------------------------------------------------

impl From<RotationDegrees> for Matrix<3, 3> {
  fn from(rotation_degrees: RotationDegrees) -> Self {
    let rotation_radians = RotationRadians::from(rotation_degrees);
    Matrix::from(rotation_radians)
  }
}

impl From<RotationDegrees> for RotationRadians {
  fn from(rotation_degrees: RotationDegrees) -> Self {
    let RotationDegrees {
      x,
      y,
      z,
    } = rotation_degrees;
    RotationRadians {
      x: x.to_radians(),
      y: y.to_radians(),
      z: z.to_radians(),
    }
  }
}

impl From<RotationRadians> for Matrix<3, 3> {
  fn from(rotation_radians: RotationRadians) -> Self {
    let RotationRadians {
      x,
      y,
      z,
    } = rotation_radians;
    let cx: f64 = x.cos();
    let sx: f64 = x.sin();
    let cy: f64 = y.cos();
    let sy: f64 = y.sin();
    let cz: f64 = z.cos();
    let sz: f64 = z.sin();
    Matrix {
      rows: [
        [
          cy * cz,
          -cx * sz + sx * sy * cz,
          sx * sz + cx * sy * cz,
        ],
        [
          cy * sz,
          cx * cz + sx * sy * sz,
          -sx * cz + cx * sy * sz,
        ],
        [
          -sy,
          sx * cy,
          cx * cy,
        ],
      ],
    }
  }
}

impl From<RotationRadians> for RotationDegrees {
  fn from(rotation_radians: RotationRadians) -> Self {
    let RotationRadians {
      x,
      y,
      z,
    } = rotation_radians;
    RotationDegrees {
      x: x.to_degrees(),
      y: y.to_degrees(),
      z: z.to_degrees(),
    }
  }
}
