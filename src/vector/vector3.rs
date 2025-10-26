//! 2D vector math utilities.
//!
//! This module provides a simple 3D vector (`Vec3`) type with common operations used in
//! computer graphics, ray tracing, and physics. Includes vector arithmetic, dot and cross products,
//! normalization and much more.

use std::ops::Mul;
use std::fmt;
use derive_more::{Add, Sub, Mul, Div, Neg, Constructor};
/// A 3-dimensional vector type.
///
/// Provides common vector operations such as addition, subtraction, scalar and component-wise
/// multiplication, normalization, dot and cross products.
///
/// # Examples
/// ```
/// 
/// use lars::Vec3;
/// let a = Vec3::new(1.0, 0.0, 0.0);
/// let b = Vec3::new(0.0, 1.0, 0.0);
///
/// let cross = a.cross(&b); // Vec3 { x: 0.0, y: 0.0, z: 1.0 }
/// let dot = a.dot(&b); // 0.0
/// ```
#[derive(Add, Sub, Mul, Div, Neg, Clone, Copy, Debug, PartialEq, PartialOrd, Constructor)]
pub struct Vec3 {
    /// X component of the vector.
    pub x: f64,
    /// Y component of the vector.
    pub y: f64,
    /// Z component of the vector.
    pub z: f64,
}

impl Vec3 {
    /// A zero Vector (0.0, 0.0, 0.0)
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    /// A one Vector (1.0, 1.0, 1.0)
    pub const ONE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
    /// A Unit Vector in X (1.0, 0.0, 0.0)
    pub const UNIT_X: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    /// A Unit Vector in Y (0.0, 1.0, 0.0)
    pub const UNIT_Y: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    /// A Unit Vector in Z (0.0, 0.0, 1.0)
    pub const UNIT_Z: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };


    /// Returns the **magnitude** (length) of the vector.
    ///
    /// # Examples
    /// ```
    ///  use lars::Vec3;
    /// let v = Vec3::new(3.0, 4.0, 0.0);
    /// assert_eq!(v.mag(), 5.0);
    /// ```
    pub fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns the **dot product** between `self` and another [`Vec3`].
    ///
    /// # Examples
    /// ```
    ///
    /// use lars::Vec3;
    /// let a = Vec3::new(1.0, 2.0, 3.0);
    /// let b = Vec3::new(4.0, -5.0, 6.0);
    /// assert_eq!(a.dot(&b), 12.0);
    /// ```
    pub fn dot(&self, other: &Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }


    /// Returns the **cross product** between `self` and another [`Vec3`].
    ///
    /// The cross product is perpendicular to both vectors.
    ///
    /// # Examples
    /// ```
    /// use lars::Vec3;
    /// let a = Vec3::new(1.0, 0.0, 0.0);
    /// let b = Vec3::new(0.0, 1.0, 0.0);
    /// assert_eq!(a.cross(&b), Vec3::new(0.0, 0.0, 1.0));
    /// ```
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let x = (self.y * other.z) - (self.z * other.y);
        let y = (self.z * other.x) - (self.x * other.z);
        let z = (self.x * other.y) - (self.y * other.x);
        Vec3 { x, y, z }
    }

    /// Applies a function `f` to each component (`x`, `y`, and `z`) of the vector.
    ///
    /// # Examples
    /// ```
    ///  use lars::Vec3;
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// let squared = v.map(|x| x * x);
    /// assert_eq!(squared, Vec3::new(1.0, 4.0, 9.0));
    /// ```
    pub fn map<F>(&self, f: F) -> Vec3
    where
        F: Fn(f64) -> f64,
    {
        let fx = f(self.x);
        let fy = f(self.y);
        let fz = f(self.z);
        Vec3 { x: fx, y: fy, z: fz }
    }

    /// Returns a **normalized** version of the vector (unit length).
    ///
    /// Each component is divided by the vector's magnitude.
    ///
    /// # Panics
    /// Panics if the vector has zero magnitude (division by zero).
    ///
    /// # Examples
    /// ```
    ///  use lars::Vec3;
    /// let v = Vec3::new(3.0, 0.0, 0.0);
    /// assert_eq!(v.normalize(), Vec3::new(1.0, 0.0, 0.0));
    /// ```
    pub fn normalize(&self) -> Vec3 {
        let m = self.mag();
        self.map(|i| i / m)
    }

    // All functions below this point are variations of the above functions

    /// Returns the **magnitude**  of the vector, squared.
    ///
    /// # Examples
    /// ```
    ///  use lars::Vec3;
    /// let v = Vec3::new(3.0, 4.0, 0.0);
    /// assert_eq!(v.mag_sq(), 25.0);
    /// ```
    pub fn mag_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }


}


/// Implements scalar multiplication of a vector by a float (`f64`).
///
/// This enables `f64 * Vec3` syntax.
///
/// # Examples
/// ```
///  use lars::Vec3;
/// let v = Vec3::new(1.0, 2.0, 3.0);
/// let scaled = 2.0 * v;
/// assert_eq!(scaled, Vec3::new(2.0, 4.0, 6.0));
/// ```
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3 {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}


/// displays the vector in the form (X, Y, Z)

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}


/// Returns (0.0, 0.0, 0.0)
impl Default for Vec3 {
    fn default() -> Self {
        Self::ZERO
    }
}



/// Implements **component-wise multiplication** between two [`Vec3`]s.
///
/// This is useful for operations such as color blending or per-component scaling.
///
/// # Examples
/// ```
///  use lars::Vec3;
/// let a = Vec3::new(1.0, 2.0, 3.0);
/// let b = Vec3::new(2.0, 0.5, 4.0);
/// assert_eq!(a * b, Vec3::new(2.0, 1.0, 12.0));
/// ```
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * vector.x,
            y: self.y * vector.y,
            z: self.z * vector.z,
        }
    }
}

/// Represents an RGB color with values between `0.0` and `1.0`.
/// Will eventually contain support for conversions with the image crate
///
/// Alias for [`Vec3`].
pub type Colour = Vec3;

/// Represents a 3D point in space.
///
/// Alias for [`Vec3`].
pub type Point3D = Vec3;
impl Point3D {

    /// Finds the unsigned distance between `self` and another 3D point `Other`.
    ///
    /// #examples
    ///
    /// ```
    /// use lars::{Point3D};
    /// let a = Point3D::new(1.0, 0.0, 0.0);
    /// let b = Point3D::new(0.0, 0.0, 0.0);
    /// assert_eq!(a.dist(&b), 1.0)
    /// ```
    pub fn dist(&self, other: &Point3D) -> f64 {
        (*self - *other).mag().abs()
    }
    /// Finds the unsigned distance between `self` and another 3D point `Other`, squared
    ///
    /// #examples
    ///
    /// ```
    ///
    /// use lars::Point3D;
    /// let a = Point3D::new(2.0, 0.0, 0.0);
    /// let b = Point3D::new(0.0, 0.0, 0.0);
    /// assert_eq!(a.dist_sq(&b), 4.0)
    ///
    /// ```
    pub fn dist_sq(&self, other: &Point3D) -> f64 {
        (*self - *other).mag_sq().abs()
    }

}



// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mag() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.mag(), 3.0);
    }
    #[test]
    fn test_mag_sq() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.mag_sq(), 9.0);
    }
    #[test]
    fn test_dist() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 0.0, 3.0);
        assert_eq!(a.dist(&b), 2.0);
    }
    #[test]
    fn test_dist_sq() {
        let a = Point3D::new(1.0, 2.0, 3.0);
        let b = Point3D::new(1.0, 0.0, 3.0);
        assert_eq!(a.dist_sq(&b), 4.0);
    }
    #[test]
    fn test_dot_product() {

        let a = Point3D::new(1.0, 2.0, 3.0);
        let b = Point3D::new(4.0, -5.0, 6.0);
        assert_eq!(a.dot(&b), 12.0);
    }

    #[test]
    fn test_cross_product() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(a.cross(&b), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_normalize() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let n = v.normalize();
        assert!((n.mag() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_scalar_mul() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(2.0 * v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_component_mul() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a * b, Vec3::new(2.0, 6.0, 12.0));
    }
    
    #[test]
    fn test_default() {
        let v = Vec3::default();
        assert_eq!(v, Vec3::ZERO);
    }
}