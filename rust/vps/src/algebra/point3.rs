/*
 * Point class
 */
use std::ops::{Add, Sub};
use num::Float;

use serde::{Serialize, Deserialize};

use crate::algebra::Vector3;

/// `Point3` represents a spatial location with an `x`, `y` and `z` coordinate
/// with associated operations.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Point3<T>
{
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Point3<T> 
where T: Float
{
  pub fn min(&self, other: &Point3<T>) -> Point3<T> {
    Point3::new(self.x.min(other.x), self.y.min(other.y), self.z.min(other.z))
  }

  pub fn max(&self, other: &Point3<T>) -> Point3<T> {
    Point3::new(self.x.max(other.x), self.y.max(other.y), self.z.max(other.z))
  }
}

impl<T> Point3<T>
where T: Float
{
  pub fn origin() -> Point3<T> {
    Point3 {
      x: num::NumCast::from(0.0).unwrap(),
      y: num::NumCast::from(0.0).unwrap(),
      z: num::NumCast::from(0.0).unwrap(),
    }
  }
}

impl<T> Default for Point3<T> 
where T: num::Zero
{
  /// Defines a default `Point3` located at `0, 0, 0`.
  fn default() -> Point3<T> {
    Point3 { x: T::zero(), y: T::zero(), z: T::zero() }
  }
}

impl<T> Point3<T> 
where T: Copy
{
  /// Creates a new `Point3` at the nominated coordinates
  pub fn new(x: T, y: T, z: T) -> Point3<T> {
    Point3 { x, y, z, }
  }
}

impl<T> Sub<&Point3<T>> for &Point3<T>
where T: Float
{
  type Output = Vector3<T>;
  fn sub(self, other: &Point3<T>) -> Self::Output {
    Vector3::<T>::new(self.x - other.x, self.y - other.y, self.z - other.z)
  }
}

impl<T> Sub for Point3<T> 
where T: Float
{
    type Output = Vector3<T>;
    fn sub(self, other: Point3<T>) -> Self::Output {
        Vector3::<T>::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T> Sub<Point3<T>> for &Point3<T>
where T: Float
{
    type Output = Vector3<T>;
    fn sub(self, other: Point3<T>) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T> Add<Vector3<T>> for Point3<T>
where T: Float + std::ops::AddAssign
{
  type Output = Point3<T>;
  fn add(mut self, other: Vector3<T>) -> Self::Output {
    self.x += other.x; self.y += other.y; self.z += other.z;
    self
  }
}

impl<T> Sub<Vector3<T>> for Point3<T>
where T: Float + std::ops::SubAssign
{
  type Output = Point3<T>;
  fn sub(mut self, other: Vector3<T>) -> Self::Output {
    self.x -= other.x; self.y -= other.y; self.z -= other.z;
    self
  }
}

impl<T> Add<&Vector3<T>> for &Point3<T>
where T: Float
{
    type Output = Point3<T>;
    fn add(self, other: &Vector3<T>) -> Self::Output {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T> Sub<&Vector3<T>> for &Point3<T>
where T: Float
{
    type Output = Point3<T>;
    fn sub(self, other: &Vector3<T>) -> Self::Output {
        Point3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

