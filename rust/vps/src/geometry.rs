use crate::{Vorton, Point3, Vector3};

mod sphere; pub use sphere::Sphere;
mod cube; pub use cube::Cube;

pub trait GeometryTrait {
  fn step(&mut self, _time_step: f64) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
  fn bounding_box(&self) -> Result<Option<(Point3<f64>, Point3<f64>)>, Box<dyn std::error::Error>>;
  fn is_inside(&self, point: &Point3<f64>) -> Result<bool, Box<dyn std::error::Error>>;
  fn intersect(&self, start: &Point3<f64>, end: &Point3<f64>) -> Result<Option<Point3<f64>>, Box<dyn std::error::Error>>;
  fn enforce<F>(&self, f: F) -> Result<Vec<Vorton>, Box<dyn std::error::Error>>
  where F: Fn(&Point3<f64>) -> Result<Vector3<f64>, Box<dyn std::error::Error>>;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Geometry {
  Sphere(Sphere),
  Cube(Cube),
}

impl Geometry {
  pub fn cube() -> Geometry { Geometry::Cube(Cube::default()) }

  pub fn step(&mut self, time_step: f64) -> Result<(), Box<dyn std::error::Error>> {
    match self {
      Geometry::Sphere(sphere) => sphere.step(time_step),
      Geometry::Cube(cube) =>     cube.step(time_step),
    }
  }

  pub fn bounding_box(&self) -> Result<Option<(Point3<f64>, Point3<f64>)>, Box<dyn std::error::Error>> {
    match self {
      Geometry::Sphere(sphere) => sphere.bounding_box(),
      Geometry::Cube(cube) =>     cube.bounding_box(),
    }
  }

  pub fn is_inside(&self, point: &Point3<f64>) -> Result<bool, Box<dyn std::error::Error>> {
    match self {
      Geometry::Sphere(sphere) => sphere.is_inside(point),
      Geometry::Cube(cube) =>     cube.is_inside(point),
    }
  }

  pub fn intersect(&self, start: &Point3<f64>, end: &Point3<f64>) -> Result<Option<Point3<f64>>, Box<dyn std::error::Error>> {
    match self {
      Geometry::Sphere(sphere) => sphere.intersect(start, end),
      Geometry::Cube(cube) =>     cube.intersect(start, end),
    }
  }

  pub fn enforce<F>(&self, f: F) -> Result<Vec<Vorton>, Box<dyn std::error::Error>> 
  where F: Fn(&Point3<f64>) -> Result<Vector3<f64>, Box<dyn std::error::Error>>
  {
    match self {
      Geometry::Sphere(sphere) => sphere.enforce(f),
      Geometry::Cube(cube) =>     cube.enforce(f),
    }
  }

}

