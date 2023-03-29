use crate::{Point3, Vector3};

pub trait VortonToVelocity {
  fn velocity_at(&self, position: &Point3<f64>) -> Result<Vector3<f64>, Box<dyn std::error::Error>>;
}
