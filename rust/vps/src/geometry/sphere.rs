use crate::{Vorton, Point3, Vector3};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Sphere {
  center: Point3<f64>,
  radius: f64,
}

impl super::GeometryTrait for Sphere {
  fn bounding_box(&self) -> Result<Option<(Point3<f64>, Point3<f64>)>, Box<dyn std::error::Error>> {
    Ok(Some(( &self.center + &Vector3::new(-1.0, -1.0, -1.0).normalize().scale(self.radius), 
              &self.center + &Vector3::new(1.0, 1.0, 1.0).normalize().scale(self.radius) )))
  }

  fn is_inside(&self, point: &Point3<f64>) -> Result<bool, Box<dyn std::error::Error>> {
    let r = ((point - &self.center).norm() - self.radius) <= 0.0;
    Ok(r)
  }

  fn intersect(&self, start: &Point3<f64>, end: &Point3<f64>) -> Result<Option<Point3<f64>>, Box<dyn std::error::Error>> {
    if self.is_inside(start)? != self.is_inside(end)? {
      Err("Need to implement algorithm to calculate intersection".into())
    } else {
      Ok(None)
    }
  }

  fn enforce<F>(&self, f: F) -> Result<Vec<Vorton>, Box<dyn std::error::Error>> 
  where F: Fn(&Point3<f64>) -> Result<Vector3<f64>, Box<dyn std::error::Error>>
  {
    Ok(Vec::new())
  }
}
