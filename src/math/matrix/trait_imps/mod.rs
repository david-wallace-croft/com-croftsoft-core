// =============================================================================
//! - Trait implementations for the structure Matrix
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

#[cfg(test)]
mod test;

use super::structures::*;

// Trait Default ---------------------------------------------------------------------

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

// Trait From ------------------------------------------------------------------------

impl From<Degrees> for Radians {
  fn from(degrees: Degrees) -> Self {
    Radians(degrees.0.to_radians())
  }
}

impl From<Matrix<3, 3>> for RotationDegrees {
  fn from(rotation_matrix: Matrix<3, 3>) -> Self {
    RotationRadians::from(rotation_matrix).into()
  }
}

impl From<Matrix<3, 3>> for RotationRadians {
  fn from(rotation_matrix: Matrix<3, 3>) -> Self {
    // Adapted from Dunn and Parberry, 3D Math Primer, 2002, page 204.
    let sp = -rotation_matrix.get_entry(Indices {
      row: 1,
      column: 2,
    });
    let heading;
    let pitch;
    let bank;
    if sp.abs() > 0.99999 {
      heading = -rotation_matrix
        .get_entry(Indices {
          row: 2,
          column: 0,
        })
        .atan2(rotation_matrix.get_entry(Indices {
          row: 0,
          column: 0,
        }));
      pitch = core::f64::consts::FRAC_PI_2 * sp;
      bank = 0.0;
    } else {
      heading = rotation_matrix
        .get_entry(Indices {
          row: 0,
          column: 2,
        })
        .atan2(rotation_matrix.get_entry(Indices {
          row: 2,
          column: 2,
        }));
      pitch = sp.asin();
      bank = rotation_matrix
        .get_entry(Indices {
          row: 1,
          column: 0,
        })
        .atan2(rotation_matrix.get_entry(Indices {
          row: 1,
          column: 1,
        }));
    }
    RotationRadians {
      x: pitch,
      y: heading,
      z: bank,
    }
  }
}

impl From<Radians> for Degrees {
  fn from(radians: Radians) -> Self {
    Degrees(radians.0.to_degrees())
  }
}

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
