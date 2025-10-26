//! 2×2 Matrix utilities.
//!
//! Provides a small, self-contained 2×2 matrix type [`Mat2`] with
//! support for basic linear algebra operations, including addition,
//! subtraction, scalar and matrix multiplication, and inversion.
//!
//! This type is designed to pair naturally with the [`Vec2`] struct
//! for 2D linear transformations.

use std::ops::Mul;
use derive_more::{Constructor, Add, Sub};
use crate::Vec2;

/// A 2×2 matrix of `f64` values.
///
/// The matrix is stored in **row-major order**:
///
/// ```text
/// | a  b |
/// | c  d |
/// ```
///
/// # Examples
/// ```
/// use lars::{Mat2, Vec2};
///
/// let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
/// let v = Vec2::new(1.0, 1.0);
///
/// assert_eq!(m * v, Vec2::new(3.0, 7.0));
/// ```
#[derive(Constructor, Copy, Clone, Debug, Add, Sub, PartialEq, PartialOrd)]
pub struct Mat2 {
    /// Top-left element.
    pub a: f64,
    /// Top-right element.
    pub b: f64,
    /// Bottom-left element.
    pub c: f64,
    /// Bottom-right element.
    pub d: f64,
}

impl Mat2 {
    /// The **identity matrix**:
    ///
    /// ```text
    /// | 1  0 |
    /// | 0  1 |
    /// ```
    pub const IDENTITY: Mat2 = Mat2::new(1.0, 0.0, 0.0, 1.0);

    /// The **zero matrix**:
    ///
    /// ```text
    /// | 0  0 |
    /// | 0  0 |
    /// ```
    pub const ZERO: Mat2 = Mat2::new(0.0, 0.0, 0.0, 0.0);

    /// Returns the **determinant** of the matrix.
    ///
    /// Computed as:
    /// \[
    /// \det(M) = ad - bc
    /// \]
    ///
    /// # Examples
    /// ```
    /// use lars::Mat2;
    /// let m = Mat2::new(7.0, 2.0, 6.0, 2.0);
    /// assert_eq!(m.determinant(), 2.0);
    /// ```
    pub fn determinant(&self) -> f64 {
        self.a * self.d - self.b * self.c
    }

    /// Returns the **inverse** of the matrix, if it exists.
    ///
    /// Computed as:
    /// \[
    /// M^{-1} = \frac{1}{\det(M)} \begin{bmatrix} d & -b \\ -c & a \end{bmatrix}
    /// \]
    ///
    /// # Panics
    /// Panics if the matrix is singular (determinant = 0).
    ///
    /// # Examples
    /// ```
    /// use lars::Mat2;
    /// let m = Mat2::new(7.0, 2.0, 6.0, 2.0);
    /// assert_eq!(m.inverse(), Mat2::new(1.0, -1.0, -3.0, 3.5));
    /// ```
    pub fn inverse(&self) -> Mat2 {
        let rec_det = 1.0 / self.determinant();
        rec_det * Mat2::new(self.d, -self.b, -self.c, self.a)
    }
}

/// Implements **matrix–scalar multiplication** (`Mat2 * f64`).
///
/// Each element of the matrix is scaled by the scalar.
///
/// # Examples
/// ```
/// use lars::Mat2;
/// let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
/// assert_eq!(m * 2.0, Mat2::new(2.0, 4.0, 6.0, 8.0));
/// ```
impl Mul<f64> for Mat2 {
    type Output = Mat2;
    fn mul(self, s: f64) -> Mat2 {
        Mat2::new(self.a * s, self.b * s, self.c * s, self.d * s)
    }
}

/// Implements **scalar–matrix multiplication** (`f64 * Mat2`).
///
/// # Examples
/// ```
/// use lars::Mat2;
/// let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
/// assert_eq!(2.0 * m, Mat2::new(2.0, 4.0, 6.0, 8.0));
/// ```
impl Mul<Mat2> for f64 {
    type Output = Mat2;
    fn mul(self, s: Mat2) -> Mat2 {
        Mat2::new(s.a * self, s.b * self, s.c * self, s.d * self)
    }
}

/// Implements **matrix–vector multiplication** (`Mat2 * Vec2`).
///
/// Performs the linear transformation of the vector by the matrix.
///
/// \[
/// \begin{bmatrix} a & b \\ c & d \end{bmatrix}
/// \begin{bmatrix} x \\ y \end{bmatrix}
/// =
/// \begin{bmatrix} ax + by \\ cx + dy \end{bmatrix}
/// \]
///
/// # Examples
/// ```
/// use lars::{Mat2, Vec2};
/// let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
/// let v = Vec2::new(1.0, 1.0);
/// assert_eq!(m * v, Vec2::new(3.0, 7.0));
/// ```
impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    fn mul(self, v: Vec2) -> Vec2 {
        let x = self.a * v.x + self.b * v.y;
        let y = self.c * v.x + self.d * v.y;
        Vec2::new(x, y)
    }
}

/// Implements **matrix–matrix multiplication** (`Mat2 * Mat2`).
///
/// Standard linear algebra multiplication:
///
/// \[
/// M_1 \times M_2 =
/// \begin{bmatrix}
/// a_1a_2 + b_1c_2 & a_1b_2 + b_1d_2 \\
/// c_1a_2 + d_1c_2 & c_1b_2 + d_1d_2
/// \end{bmatrix}
/// \]
///
/// # Examples
/// ```
/// use lars::Mat2;
/// let a = Mat2::IDENTITY;
/// let b = Mat2::new(1.0, 2.0, 3.0, 4.0);
/// assert_eq!(a * b, b);
/// ```
impl Mul<Mat2> for Mat2 {
    type Output = Mat2;
    fn mul(self, m: Mat2) -> Mat2 {
        let a = self.a * m.a + self.b * m.c;
        let b = self.a * m.b + self.b * m.d;
        let c = self.c * m.a + self.d * m.c;
        let d = self.c * m.b + self.d * m.d;
        Mat2::new(a, b, c, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m + m, Mat2::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn sub() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m - m, Mat2::ZERO);
    }

    #[test]
    fn test_s_mul1() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(m * 1.0, m)
    }

    #[test]
    fn test_s_mul2() {
        let m = Mat2::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(1.0 * m, m)
    }

    #[test]
    fn test_v_mul1() {
        let i = Mat2::IDENTITY;
        let v = Vec2::new(1.0, 1.0);
        assert_eq!(i * v, v)
    }

    #[test]
    fn test_mat_mul() {
        let a = Mat2::IDENTITY;
        let b = Mat2::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(a * b, b);
    }

    #[test]
    fn test_determinant() {
        let m = Mat2::new(7.0, 2.0, 6.0, 2.0);
        assert_eq!(m.determinant(), 2.0);
    }

    #[test]
    fn test_inverse() {
        let m = Mat2::new(7.0, 2.0, 6.0, 2.0);
        assert_eq!(m.inverse(), Mat2::new(1.0, -1.0, -3.0, 3.5))
    }
}
