use crate::{Point3, Vector3, Vorton, GeometryTrait};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Cube {}

impl super::GeometryTrait for Cube {
  fn bounding_box(&self) -> Result<Option<(Point3<f64>, Point3<f64>)>, Box<dyn std::error::Error>> {
    Ok(Some((Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0))))
  }

  fn is_inside(&self, point: &Point3<f64>) -> Result<bool, Box<dyn std::error::Error>> {
    let v = point - &Point3::new(0.0, 0.0, 0.0);
    Ok(
       0.0 <= v.x && v.x <= 1.0
    && 0.0 <= v.y && v.y <= 1.0
    && 0.0 <= v.z && v.z <= 1.0
    )
  }

  fn intersect(&self, start: &Point3<f64>, end: &Point3<f64>) -> Result<Option<Point3<f64>>, Box<dyn std::error::Error>> {
    if self.is_inside(start)? != self.is_inside(end)? {
      Err("Algorithm to be implemented".into())
    } else {
      Ok(None)
    }
  }

  fn enforce<F>(&self, f: F) -> Result<Vec<Vorton>, Box<dyn std::error::Error>> 
  where F: Fn(&Point3<f64>) -> Result<Vector3<f64>, Box<dyn std::error::Error>>
  {
    let n = 4; let delta = 1.0 / n as f64;
    let mut r = Vec::new();
    // front face x = 0
    for i in 0..n {
      for j in 0..n {
        let p = Point3::new(0.0, (i as f64 + 0.5)*delta, (j as f64 + 0.5)*delta);
    let n = Vector3::new(-1.0, 0.0, 0.0); let t1 = Vector3::new(0.0, 1.0, 0.0);
        if let Some(v) = self.correct_at(&p, &n, &t1, &Vector3::new(0.0, 0.0, 0.0), &f)? { r.push(v); }
      }
    }
    // Back face x = 1
    for i in 0..n {
      for j in 0..n {
        let p = Point3::new(1.0, (i as f64 + 0.5)*delta, (j as f64 + 0.5)*delta);
    let n = Vector3::new(1.0, 0.0, 0.0); let t1 = Vector3::new(0.0, 1.0, 0.0);
        if let Some(v) = self.correct_at(&p, &n, &t1, &Vector3::new(0.0, 0.0, 0.0), &f)? { r.push(v); }
      }
    }
    // Face y = 0
    for i in 0..n {
      for j in 0..n {
        let p = Point3::new((i as f64 + 0.5)*delta, 0.0, (j as f64 + 0.5)*delta);
    let n = Vector3::new(0.0,-1.0, 0.0); let t1 = Vector3::new(1.0, 0.0, 0.0);
        if let Some(v) = self.correct_at(&p, &n, &t1, &Vector3::new(0.0, 0.0, 0.0), &f)? { r.push(v); }
      }
    }
    // Face y = 1
    for i in 0..n {
      for j in 0..n {
        let p = Point3::new((i as f64 + 0.5)*delta, 1.0, (j as f64 + 0.5)*delta);
    let n = Vector3::new(0.0, 1.0, 0.0); let t1 = Vector3::new(1.0, 0.0, 0.0);
        if let Some(v) = self.correct_at(&p, &n, &t1, &Vector3::new(0.0, 0.0, 0.0), &f)? { r.push(v); }
      }
    }
    // Face z = 0
    for i in 0..n {
      for j in 0..n {
        let p = Point3::new((i as f64 + 0.5)*delta, (j as f64 + 0.5)*delta, 0.0);
    let n = Vector3::new(0.0, 0.0,-1.0); let t1 = Vector3::new(1.0, 0.0, 0.0);
        if let Some(v) = self.correct_at(&p, &n, &t1, &Vector3::new(0.0, 0.0, 0.0), &f)? { r.push(v); }
      }
    }
    // Face z = 1
    for i in 0..n {
      for j in 0..n {
        let p = Point3::new((i as f64 + 0.5)*delta, (j as f64 + 0.5)*delta, 1.0);
    let n = Vector3::new(0.0, 0.0, 1.0); let t1 = Vector3::new(1.0, 0.0, 0.0);
        if let Some(v) = self.correct_at(&p, &n, &t1, &Vector3::new(0.0, 0.0, 0.0), &f)? { r.push(v); }
      }
    }

    Ok(r)
  }
}

impl Cube {
  /// Generate a vorton that correct the velocity at the contact point to meet the provided value
  fn correct_at<F>(&self, 
                contact_point: &Point3<f64>, 
                normal: &Vector3<f64>, 
                tangent1: &Vector3<f64>,
                value: &Vector3<f64>,
                f: &F) -> Result<Option<Vorton>, Box<dyn std::error::Error>> 
  where F: Fn(&Point3<f64>) -> Result<Vector3<f64>, Box<dyn std::error::Error>>
  {
    let current = f(contact_point)?;
    let correction = (current + value.scale(-1.0)).scale(-1.0);
    if correction.norm().abs() < 1e-6 { return Ok(None); }

    let distance: f64 = 0.02; let radius = distance / 4.0; let volume = 4.0 / 3.0 * std::f64::consts::PI * radius.powi(3);

    // Find where to put the new vorton
    let d = correction.normalize();
    let direction = 
      if (normal.dot(&d).abs() - 1.0).abs() < 1e-6 {
        tangent1.clone()
      } else {
        normal.cross(&d).normalize().cross(&d)
      };
    let mut position = contact_point + &direction.scale(distance);
    if self.is_inside(&position)? { position = contact_point + &direction.scale(-distance); }
    Ok(Some( Vorton::make_velocity_at(position, volume, (contact_point, &correction))? ))
  }

  /// Generate a vorton based on the vorticity at a point next to the boundary
  fn correct_at_v1<F>(contact_point: &Point3<f64>, 
                normal: &Vector3<f64>, 
                tangent1: &Vector3<f64>,
                value: &Vector3<f64>,
                f: &F) -> Result<Option<Vorton>, Box<dyn std::error::Error>> 
  where F: Fn(&Point3<f64>) -> Result<Vector3<f64>, Box<dyn std::error::Error>>
  {
    if f(contact_point)?.norm().abs() < 1e-6 { return Ok(None); }


    let distance: f64 = 0.01; let radius = distance / 4.0; let volume = 4.0 / 3.0 * std::f64::consts::PI * radius.powi(3);
    let position = contact_point + &normal.scale(0.5 * distance);
    let test_point = contact_point + &normal.scale(distance);

    let tangent2 = normal.cross(&tangent1);
    let convert = |v: &Vector3<f64>| {Vector3::new(v.dot(normal), v.dot(tangent1), v.dot(&tangent2))};
    
    let velocity = convert(&f(&test_point)?);
    let derivative_distance = 0.001;
    let velocity_tangent1 = convert(&f(&(&test_point + &tangent1.scale(derivative_distance)))?);
    let velocity_tangent2 = convert(&f(&(&test_point + &tangent2.scale(derivative_distance)))?);

    let vorticity 
    =   normal.scale(0.5*(velocity_tangent1.z - velocity.z) / derivative_distance - 0.5*(velocity_tangent2.y - velocity.y) / derivative_distance)
    + tangent1.scale(0.5*(velocity_tangent2.x - velocity.x) / derivative_distance - (velocity.z - value.z) / distance)
    + tangent2.scale(    (velocity.y - value.y) / distance                        - 0.5*(velocity_tangent1.x - velocity.x) / derivative_distance);

    Ok(Some( Vorton::new(position, vorticity, volume) ))
  }
}

