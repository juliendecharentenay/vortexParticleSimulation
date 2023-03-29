use crate::{Point3, Vorton};

#[derive(Debug, Clone)]
pub struct SuperVorton {
  vorton: Vorton,
  max_vorticity: f64,
  cum_vorticity: f64,
  n_vortons: usize,
}

impl SuperVorton {
  pub fn vorton(&self) -> &Vorton    { &self.vorton }
  pub fn max_vorticity(&self) -> f64 { self.max_vorticity }
  pub fn n_vortons(&self) -> usize   { self.n_vortons }
}

impl Default for SuperVorton {
  fn default() -> Self {
    SuperVorton {
      vorton: Vorton::default(),
      max_vorticity: 0.0,
      cum_vorticity: 0.0,
      n_vortons: 0,
    }
  }
}

impl From<&Vorton> for SuperVorton {
  fn from(v: &Vorton) -> Self {
    SuperVorton {
      vorton: v.clone(),
      max_vorticity: v.vorticity().norm(),
      cum_vorticity: v.vorticity().norm(),
      n_vortons: 1,
    }
  }
}

impl From<&Vec<&Vorton>> for SuperVorton {
  fn from(v: &Vec<&Vorton>) -> Self {
    v.iter().fold(SuperVorton::default(), |r, v| r + *v)
  }
}

impl std::ops::Add<&Vorton> for &Vorton {
  type Output = SuperVorton;
  fn add(self, other: &Vorton) -> Self::Output {
    SuperVorton::default() + self + other
  }
}

impl std::ops::Add<&Vorton> for SuperVorton {
  type Output = SuperVorton;
  fn add(self, other: &Vorton) -> Self::Output {
    &self + other
  }
}

impl std::ops::Add<&Vorton> for &SuperVorton {
  type Output = SuperVorton;
  fn add(self, other: &Vorton) -> Self::Output {
    let volume = self.vorton.volume() + other.volume();
    let cum_vorticity = (self.cum_vorticity*self.vorton.volume() + other.vorticity().norm()*other.volume())/volume;
    let max_vorticity = self.max_vorticity.max(other.vorticity().norm());
    let position = Point3::<f64>::origin()
        + ( (self.vorton.position() - &Point3::<f64>::origin()).scale(self.cum_vorticity * self.vorton.volume())
          + (other.position()       - &Point3::<f64>::origin()).scale(other.vorticity().norm() * other.volume())
          ).scale(1.0 / (self.cum_vorticity * self.vorton.volume() + other.vorticity().norm() * other.volume()));
    let vorticity = (self.vorton.vorticity().scale(self.vorton.volume()) + other.vorticity().scale(other.volume()))
         .scale(1.0 / volume);
    SuperVorton {
      vorton: Vorton::new(position, vorticity, volume),
      max_vorticity,
      cum_vorticity,
      n_vortons: self.n_vortons + 1,
    }
  }
}

impl std::ops::Add<&SuperVorton> for SuperVorton {
  type Output = SuperVorton;
  fn add(self, other: &SuperVorton) -> Self::Output {
    &self + other
  }
}

impl std::ops::Add<&SuperVorton> for &SuperVorton {
  type Output = SuperVorton;
  fn add(self, other: &SuperVorton) -> Self::Output {
    let volume = self.vorton.volume() + other.vorton.volume();
    let cum_vorticity = (self.cum_vorticity*self.vorton.volume() + other.cum_vorticity*other.vorton.volume())/volume;
    let max_vorticity = self.max_vorticity.max(other.max_vorticity);
    let position = Point3::<f64>::origin()
        + ( (self.vorton.position()  - &Point3::<f64>::origin()).scale(self.cum_vorticity  * self.vorton.volume())
          + (other.vorton.position() - &Point3::<f64>::origin()).scale(other.cum_vorticity * other.vorton.volume())
          ).scale(1.0 / (self.cum_vorticity * self.vorton.volume() + other.cum_vorticity * other.vorton.volume()));
    let vorticity = (self.vorton.vorticity().scale(self.vorton.volume()) + other.vorton.vorticity().scale(other.vorton.volume()))
         .scale(1.0 / volume);
    SuperVorton {
      vorton: Vorton::new(position, vorticity, volume),
      max_vorticity,
      cum_vorticity,
      n_vortons: self.n_vortons + other.n_vortons,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{Vector3};

  #[test]
  fn it_adds() -> Result<(), Box<dyn std::error::Error>> {
    let v1 = Vorton::new(Point3::new( 1.0, 0.0, 0.0), Vector3::new(-1.0, 0.0, 0.0), 0.1);
    let v2 = Vorton::new(Point3::new( 3.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 0.1);
    let v = &v1 + &v2;
    println!("#1 v: {v:#?}");
    assert!((v.vorton().position().x.abs() - 2.0) < 1e-6);


    let v1 = Vorton::new(Point3::new(-3.0, 0.0, 0.0), Vector3::new(-1.0, 0.0, 0.0), 0.05);
    let v2 = Vorton::new(Point3::new( 3.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 0.05);
    let v3 = Vorton::new(Point3::new( 3.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 0.1);
    let v = &v1 + &v2;
    println!("v: {v:#?}");
    println!("v3: {v3:#?}");
    let v = &v + &v3;
    println!("v: {v:#?}");
    assert!((v.vorton().position().x - 1.0).abs() < 1e-6);
    assert!((v.vorton().volume() - 0.2).abs() < 1e-6);
    Ok(())
  }
}

