//! 2D vector math utilities.
//!
//! This module provides a simple 2D vector type (`Vec2`) with common mathematical
//! operations used in geometry, graphics, and game development.
//!
//! It supports vector addition, subtraction, scaling, dot and cross products, and normalization.
use std::ops::Mul;
use derive_more::{Add, Sub, Mul, Div, Neg, Constructor};
use super::scalar::Scalar;


/// A 2-dimensional vector.
///
/// Provides common vector operations such as addition, subtraction, scalar and
/// component-wise multiplication, normalization, dot and cross products.
///
/// # Examples
/// ```
///
/// use lars::Vec2;
/// let a = Vec2::new(3.0, 4.0);
/// assert_eq!(a.mag(), 5.0);
/// ```
#[derive(Add, Sub, Div, Mul, Neg, Clone, Copy, Debug, PartialEq, PartialOrd, Constructor)]
pub struct Vec2 {
    /// X component of the vector.
    pub x: f64,
    /// Y component of the vector.
    pub y: f64,
}

impl Vec2 {

    /// Returns the **magnitude** (length) of the vector.
    ///
    /// # Examples
    /// ```
    ///
    /// use lars::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// assert_eq!(v.mag(), 5.0);
    /// ```
    pub fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns the **dot product** between `self` and another [`Vec2`].
    ///
    /// The dot product measures the cosine of the angle between two vectors.
    ///
    /// # Examples
    /// ```
    ///
    /// use lars::Vec2;
    /// let a = Vec2::new(1.0, 2.0);
    /// let b = Vec2::new(3.0, 4.0);
    /// assert_eq!(a.dot(&b), 11.0);
    /// ```
    pub fn dot(&self, other: &Vec2) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    /// Returns the **scalar 2D cross product** between `self` and another [`Vec2`].
    ///
    /// Unlike the 3D cross product, the 2D version returns a scalar equal to
    /// the signed area of the parallelogram formed by the two vectors.
    ///
    /// # Examples
    /// ```
    ///
    /// use lars::Vec2;
    /// let a = Vec2::new(1.0, 0.0);
    /// let b = Vec2::new(0.0, 1.0);
    /// assert_eq!(a.cross(&b), 1.0);
    /// ```
    pub fn cross(&self, other: &Vec2) -> Scalar {
        self.x * other.y - self.y * other.x
    }

    /// Applies a function `f` to each component (`x`, `y`) of the vector.
    ///
    /// # Examples
    /// ```
    ///
    /// use lars::Vec2;
    /// let v = Vec2::new(1.0, 2.0);
    /// let squared = v.map(|x| x * x);
    /// assert_eq!(squared, Vec2::new(1.0, 4.0));
    /// ```
    pub fn map<F>(&self, f: F) -> Vec2
    where
        F: Fn(f64) -> f64,
    {
        let fx = f(self.x);
        let fy = f(self.y);
        Vec2 { x: fx, y: fy }
    }

    /// Returns a **normalized** (unit-length) version of the vector.
    ///
    /// # Panics
    /// Panics if the vector has zero magnitude (division by zero).
    ///
    /// # Examples
    /// ```
    ///
    /// use lars::Vec2;
    /// let v = Vec2::new(3.0, 0.0);
    /// assert_eq!(v.normalize(), Vec2::new(1.0, 0.0));
    /// ```
    pub fn normalize(&self) -> Vec2 {
        let m = self.mag();
        self.map(|i| i / m)
    }

    // All functions below this point are variations of the above functions

    ///  Returns the **magnitude**  of the vector, squared.
    ///
    /// # Examples
    /// ```
    ///
    /// use lars::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// assert_eq!(v.mag_sq(), 25.0);
    /// ```
    pub fn mag_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }



}

/// Implements **scalar multiplication** for `f64 * Vec2`.
///
/// # Examples
/// ```
///
/// use lars::Vec2;
/// let v = Vec2::new(1.0, 2.0);
/// let scaled = 2.0 * v;
/// assert_eq!(scaled, Vec2::new(2.0, 4.0));
/// ```
impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, vector: Vec2) -> Vec2 {
        Vec2 {
            x: self * vector.x,
            y: self * vector.y,
        }
    }
}

/// Implements **component-wise multiplication** between two [`Vec2`]s.
///
/// # Examples
/// ```
///
/// use lars::Vec2;
/// let a = Vec2::new(1.0, 2.0);
/// let b = Vec2::new(3.0, 4.0);
/// assert_eq!(a * b, Vec2::new(3.0, 8.0));
/// ```
impl Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, vector: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * vector.x,
            y: self.y * vector.y,
        }
    }
}

/// Represents a 2D point in space.
///
/// Alias for [`Vec2`].
pub type Point2D = Vec2;

impl Point2D {

    /// Finds the unsigned distance between `self` and another 3D point `Other`.
    ///
    /// #examples
    ///
    /// ```
    /// use lars::Point2D;
    /// let a = Point2D::new(1.0, 0.0);
    /// let b = Point2D::new(0.0, 0.0);
    /// assert_eq!(a.dist(&b), 1.0)
    ///
    ///
    ///
    ///
    /// ```
    pub fn dist(&self, other: &Point2D) -> f64 {
        (*self - *other).mag().abs()
    }
    /// Finds the unsigned distance between `self` and another 3D point `Other`, squared
    ///
    /// #examples
    ///
    /// ```
    ///
    /// use lars::Point2D;
    /// let a = Point2D::new(2.0, 0.0);
    /// let b = Point2D::new(0.0, 0.0);
    /// assert_eq!(a.dist_sq(&b), 4.0)
    ///
    /// ```
    pub fn dist_sq(&self, other: &Point2D) -> f64 {
        (*self - *other).mag_sq().abs()
    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mag() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.mag(), 5.0);
    }

    #[test]
    fn test_mag_sq() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.mag_sq(), 25.0);
    }

    #[test]
    fn test_dist() {
        let a = Point2D::new(1.0, 2.0);
        let b = Point2D::new(1.0, 0.0);
        assert_eq!(a.dist(&b), 2.0);
    }

    #[test]
    fn test_dist_sq() {
        let a = Point2D::new(1.0, 2.0);
        let b = Point2D::new(1.0, 0.0);
        assert_eq!(a.dist_sq(&b), 4.0);
    }


    #[test]
    fn test_dot() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a.dot(&b), 11.0);
    }


    #[test]
    fn test_cross() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, 1.0);
        assert_eq!(a.cross(&b), 1.0);
    }

    #[test]
    fn test_normalize() {
        let v = Vec2::new(3.0, 4.0);
        let n = v.normalize();
        assert!((n.mag() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_scalar_mul() {
        let v = Vec2::new(1.0, 2.0);
        assert_eq!(2.0 * v, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn test_component_mul() {
        let a = Vec2::new(2.0, 3.0);
        let b = Vec2::new(4.0, 5.0);
        assert_eq!(a * b, Vec2::new(8.0, 15.0));
    }
}