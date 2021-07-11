/*
 * Point class
 */
use std::ops::{Add, Sub, Mul};
use num::Float;

use serde::{Serialize, Deserialize};

use crate::algebra::Vector3;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Point3<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T> 
{
    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3 { x, y, z, }
    }
}

impl<T> Sub for Point3<T> 
where T: Float
{
    type Output = Vector3<T>;

    fn sub(self, other: Self) -> Self::Output {
        Vector3::<T>::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<'a, T> Sub<Point3<T>> for &'a Point3<T>
where T: Float
{
    type Output = Vector3<T>;
    fn sub(self, other: Point3<T>) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}


impl<T> Add<Vector3<T>> for Point3<T>
where T: Float
{
    type Output = Point3<T>;
    fn add(self, other: Vector3<T>) -> Self::Output {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T> Sub<Vector3<T>> for Point3<T>
where T: Float
{
    type Output = Point3<T>;
    fn sub(self, other: Vector3<T>) -> Self::Output {
        Point3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

