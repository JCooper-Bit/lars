//! 3×3 Matrix utilities.
//!
//! Provides a small, self-contained 3×3 matrix type [`Mat3`] with
//! support for basic linear algebra operations, including addition,
//! subtraction, scalar and matrix multiplication, and inversion.
//!
//! This type is designed to pair naturally with the [`Vec3] struct
//! for 3D linear transformations.

use std::ops::Mul;
use derive_more::{Constructor, Add, Sub, Div};
use crate::Vec3;

/// a 3×3 matrix of `f64` values.
///
/// The matrix is stored in **row-major order**:
///
/// ```text
/// | a  b  c |
/// | c  d  e |
/// | g  h  i |
/// ```
///

/// ```
#[derive(Constructor, Copy, Clone, Debug, Add, Sub, PartialOrd, Div)]
pub struct Mat3 {
    /// First row, first column element.
    pub a: f64,
    /// First row, second column element.
    pub b: f64,
    /// First row, third column element.
    pub c: f64,
    /// Second row, first column element.
    pub d: f64,
    /// Second row, second column element.
    pub e: f64,
    /// Second row, third column element.
    pub f: f64,
    /// Third row, first column element.
    pub g: f64,
    /// Third row, second column element.
    pub h: f64,
    /// Third row, third column element.
    pub i: f64,
}

impl Mat3 {
    /// The **identity matrix**:
    ///
    /// ```text
    /// | 1  0  0 |
    /// | 0  1  0 |
    /// | 0  0  1 |
    /// ```
    pub const IDENTITY: Mat3 = Mat3 {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 1.0,
        f: 0.0,
        g: 0.0,
        h: 0.0,
        i: 1.0,
    };
    /// The **zero matrix**:
    ///
    /// ```text
    /// | 0  0  0 |
    /// | 0  0  0 |
    /// | 0  0  0 |
    /// ```
    pub const ZERO: Mat3 = Mat3 {
        a: 0.0,
        b: 0.0,
        c: 0.0,
        d: 0.0,
        e: 0.0,
        f: 0.0,
        g: 0.0,
        h: 0.0,
        i: 0.0,
    };
    /// Returns the **determinant** of the matrix.
    ///
    /// Computed as:
    /// \[
    /// \det(M) = a(ei - fh) - b(di - fg) + c(dh - eg)
    /// \]
    ///
    /// # Examples
    /// ```
    /// use lars::Mat3;
    /// let m = Mat3::new(1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 1.0, 3.0);
    /// assert_eq!(m.determinant(), -12.0);
    /// ```
    pub fn determinant(&self) -> f64 {
        self.a * (self.e * self.i - self.f * self.h) - self.b * (self.d * self.i - self.f * self.g) + self.c * (self.d * self.h - self.e * self.g)
    }
    /// Returns the **inverse** of the matrix, if it exists.
    ///
    /// Computed as:
    /// M⁻¹ = (1/det(M)) * adj(M)
    ///
    ///          1        | ei - fh   ch - bi   bf - ce |
    /// M⁻¹ = -------  x  | fg - di   ai - cg   cd - af |
    ///        det(M)     | dh - eg   bg - ah   ae - bd |
    ///
    /// # Panics
    /// Panics if the matrix is singular (determinant = 0).
    ///
    /// # Examples
    /// ```
    /// use lars::Mat3;
    /// let m = Mat3::new(1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 1.0, 3.0);
    /// assert_eq!(m.inverse(), Mat3::new(-5.0, 3.0, 4.0, 7.0, 3.0, -8.0, 1.0, -3.0, 4.0)/12.0);
    pub fn inverse(&self) -> Mat3 {
        let det = self.determinant();
        if det == 0.0 {
            panic!("Matrix is singular and cannot be inverted.");
        }
        let inv_det = 1.0 / det;

        Mat3 {
            a: (self.e * self.i - self.f * self.h) * inv_det,
            b: (self.c * self.h - self.b * self.i) * inv_det,
            c: (self.b * self.f - self.c * self.e) * inv_det,
            d: (self.f * self.g - self.d * self.i) * inv_det,
            e: (self.a * self.i - self.c * self.g) * inv_det,
            f: (self.c * self.d - self.a * self.f) * inv_det,
            g: (self.d * self.h - self.e * self.g) * inv_det,
            h: (self.b * self.g - self.a * self.h) * inv_det,
            i: (self.a * self.e - self.b * self.d) * inv_det,
        }
    }

}

const EPSILON: f64 = 1e-9;

impl PartialEq for Mat3 {
    fn eq(&self, other: &Self) -> bool {
        (self.a - other.a).abs() < EPSILON &&
            (self.b - other.b).abs() < EPSILON &&
            (self.c - other.c).abs() < EPSILON &&
            (self.d - other.d).abs() < EPSILON &&
            (self.e - other.e).abs() < EPSILON &&
            (self.f - other.f).abs() < EPSILON &&
            (self.g - other.g).abs() < EPSILON &&
            (self.h - other.h).abs() < EPSILON &&
            (self.i - other.i).abs() < EPSILON
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.a * rhs.x + self.b * rhs.y + self.c * rhs.z,
            y: self.d * rhs.x + self.e * rhs.y + self.f * rhs.z,
            z: self.g * rhs.x + self.h * rhs.y + self.i * rhs.z,
        }
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Mat3 {
        Mat3 {
            a: self.a * rhs.a + self.b * rhs.d + self.c * rhs.g,
            b: self.a * rhs.b + self.b * rhs.e + self.c * rhs.h,
            c: self.a * rhs.c + self.b * rhs.f + self.c * rhs.i,
            d: self.d * rhs.a + self.e * rhs.d + self.f * rhs.g,
            e: self.d * rhs.b + self.e * rhs.e + self.f * rhs.h,
            f: self.d * rhs.c + self.e * rhs.f + self.f * rhs.i,
            g: self.g * rhs.a + self.h * rhs.d + self.i * rhs.g,
            h: self.g * rhs.b + self.h * rhs.e + self.i * rhs.h,
            i: self.g * rhs.c + self.h * rhs.f + self.i * rhs.i,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let m = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m + m, Mat3::new(2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0));
    }

    #[test]
    fn sub() {
        let m = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(m - m, Mat3::ZERO);
    }

    #[test]
    fn test_mat_mul() {
        let a = Mat3::IDENTITY;
        let b = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(a * b, b);
    }

    #[test]
    fn test_determinant() {
        let m = Mat3::new(1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 1.0, 3.0);
        assert_eq!(m.determinant(), -12.0);
    }

    #[test]
    fn test_inverse() {
        let m = Mat3::new(1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 1.0, 3.0);
        assert_eq!(m.inverse(), Mat3::new(-5.0, 3.0, 4.0, 7.0, 3.0, -8.0, 1.0, -3.0, 4.0)/12.0);
    }
}