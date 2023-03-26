/*
 * Vector class
 */
use std::ops::{Add, Mul, Sub};
use std::cmp::PartialOrd;
use num::{Float, NumCast};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector3<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> 
where T: Float + Mul<Output = T> + PartialOrd + Sub<Output = T>
{
    pub fn x() -> Vector3<T> { 
        Vector3 { 
            x: NumCast::from(1.0).unwrap(), 
            y: NumCast::from(0.0).unwrap(), 
            z: NumCast::from(0.0).unwrap(),
        } 
    }

    pub fn y() -> Vector3<T> { 
        Vector3 { 
            x: NumCast::from(0.0).unwrap(), 
            y: NumCast::from(1.0).unwrap(), 
            z: NumCast::from(0.0).unwrap(),
        } 
    }

    pub fn z() -> Vector3<T> { 
        Vector3 { 
            x: NumCast::from(0.0).unwrap(), 
            y: NumCast::from(0.0).unwrap(),
            z: NumCast::from(1.0).unwrap(), 
        } 
    }

    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z, }
    }

    pub fn cross(&self, v: &Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.y*v.z - self.z*v.y,
            y: self.z*v.x - self.x*v.z,
            z: self.x*v.y - self.y*v.x,
        }
    }

    pub fn scale(&self, l: T) -> Vector3<T> {
        Vector3::new(self.x*l, self.y*l, self.z*l)
    }

    pub fn normalize(&self) -> Vector3<T> {
        let l = self.norm();
        if l < NumCast::from(1e-10).unwrap() {
            Vector3::x()
        } else {
            let one: T = NumCast::from(1.0).unwrap();
            let r: T = one / l;
            self.scale(r)
        }
    }

    pub fn norm(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, o: &Vector3<T>) -> T {
        self.x*o.x + self.y*o.y + self.z*o.z
    }
}

impl<T> Add for Vector3<T> 
where T: Float + std::ops::AddAssign
{
    type Output = Self;
    fn add(mut self, other: Self) -> Self::Output {
      self.x += other.x; self.y += other.y; self.z += other.z;
      self
    }
}

impl<T> Add<Vector3<T>> for &Vector3<T>
where T: Float
{
  type Output = Vector3<T>;
  fn add(self, other: Vector3<T>) -> Self::Output {
      Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
  }
}

impl<T> Sub for Vector3<T>
where T: Float
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

