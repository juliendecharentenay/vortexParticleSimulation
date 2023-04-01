use crate::{VortonToVelocity, Vorton, Point3, Vector3};

/// Algorithm to calculate the velocity from a field of vorton
/// by going through each vorton contribution one by one.
#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct VortonToVelocitySimple<'a> {
  vortons: &'a Vec<Vorton>,
  velocity: &'a Vector3<f64>,
}

impl<'a> VortonToVelocity for VortonToVelocitySimple<'a> {
  fn velocity_at(&self, position: &Point3<f64>) -> Result<Vector3<f64>, Box<dyn std::error::Error>> {
    Ok(
    self.vortons.iter()
    .map(|v| v.velocity_contribution(position))
    .fold(self.velocity.clone(), |r, v| r + v)
    )
  }
}

