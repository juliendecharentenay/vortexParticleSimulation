use serde::{Serialize, Deserialize};

use crate::algebra::{Point3, Vector3};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vorton {
    volume: f64,
    position: Point3<f64>,
    vorticity: Vector3<f64>,
}

impl Default for Vorton {
  fn default() -> Vorton {
    Vorton {
      volume: f64::EPSILON,
      position: Point3::origin(),
      vorticity: Vector3::new(0.0, 0.0, 0.0),
    }
  }
}

impl Vorton {
    pub fn new(position: Point3<f64>, vorticity: Vector3<f64>, volume: f64) -> Vorton {
        Vorton {
            volume,
            position,
            vorticity,
        }
    }

    pub fn make_velocity_at(position: Point3<f64>, volume: f64,
               (target_point, target_velocity): (&Point3<f64>, &Vector3<f64>)) -> Result<Vorton, Box<dyn std::error::Error>> {
      let r = target_point - &position;
      let d = r.norm();
      if d < 1e-6 { 
        println!("position: {position:?}\nvolume: {volume},\ntarget_point: {target_point:?},\ntarget_velocity: {target_velocity:?}\n");
        return Err("Target and position are too close".into()); 
      }
      let vorticity_vector = r.cross(target_velocity);
      if vorticity_vector.norm().abs() < 1e-6 { return Err("Unable to make velocity, position and target points are not appropriately positioned".into()); }
      let vorticity = vorticity_vector.normalize().scale(3.0*Self::ratio(&r, volume).max(8.0) / d * target_velocity.norm());
      let r = Vorton { volume, position, vorticity } ;

      // check
/*
      let v = r.velocity_contribution(target_point);
      if (v.x - target_velocity.x).abs() > 1e-1
      || (v.y - target_velocity.y).abs() > 1e-1
      || (v.z - target_velocity.z).abs() > 1e-1 {
        println!("check: {v:?}\ntarget: {target_velocity:?}");
        return Err("Not right".into());
      }
*/
      Ok( r )
    }

    pub fn is_inside(&self, position: &Point3<f64>) -> bool {
      Self::ratio(&(position - &self.position), self.volume) <= 8.0
    }

    fn ratio(r: &Vector3<f64>, volume: f64) -> f64 {
      4.0 / 3.0 * std::f64::consts::PI * r.norm().powi(3) / volume
    }

    pub fn velocity_contribution(&self, position: &Point3<f64>) -> Vector3<f64> {
        let r = position - &self.position;
        self.vorticity
            .cross(&r)
            .scale(1.0/(3.0 * Self::ratio(&r, self.volume).max(8.0)))
    }

    pub fn advect(&self, velocity: &Vector3<f64>, time_step: f64) -> Vorton {
        Vorton::new(
            &self.position + &velocity.scale(time_step),
            self.vorticity.clone(),
            self.volume
        )
    }

    pub fn step(&self, source: &Vector3<f64>, time_step: f64) -> Vorton {
        Vorton::new(
            self.position.clone(),
            &self.vorticity + source.scale(time_step),
            self.volume
        )
    }

    pub fn volume(&self)    -> f64           { self.volume.clone() }
    pub fn vorticity(&self) -> &Vector3<f64> { &self.vorticity }
    pub fn position(&self)  -> &Point3<f64>  { &self.position }
}

/*
impl std::ops::Add<&Vorton> for &Vorton {
  type Output = Vorton;
  fn add(mut self, other: &Vorton) -> Self::Output {
    let volume = self.volume + other.volume;
    let self_vort_vol = self.vorticity.norm()*self.volume;
    let other_vort_vol = other.vorticity.norm()*other.volume;
    let position = Point3::<f64>::origin()
      + ( (&self.position - &Point3::<f64>::origin()).scale(self_vort_vol)
          + (&other.position - &Point3::<f64>::origin()).scale(other_vort_vol)
        ).scale(1.0 / (self_vort_vol + other_vort_vol));
    let vorticity = (self.vorticity.scale(self.volume)
      + other.vorticity.scale(other.volume))
      .scale(1.0/volume);
    println!("Adding {self:?} + {other:?} = {position:?}/{vorticity:?}/{volume}");
    Vorton::new(
            position,
            vorticity,
            volume
    )
  }
}
*/

/*
impl Aggregatable for Vorton {
    fn aggregate(&self, other: &Vorton) -> Vorton {
    }
}
*/

/*
#[cfg(test)]
mod test_vorton {
    use super::*;

    #[test]
    fn make_vorton() -> Result<(), Box<dyn Error>> {
        let v = Vorton::new(Position::new(0.0, 1.0, 2.0), Vector::new(2.2, 0.2, 0.5), 1e-3, 1.0, 0.0)?;
        assert_eq!(v.position().y(), 1.0);
        assert_eq!(v.vorticity().y(), 0.2);
        Ok(())
    }

    #[test]
    fn velocity_contribution() -> Result<(), Box<dyn Error>> {
        let vorton = Vorton::new(Position::new(0.0, 0.0, 0.0), Vector::new(1.5, 0.0, 0.0), 1e-3, 1.0, 0.0)?;
        let mut v = vorton.velocity_contribution(&Position::new(0.0, 0.0, 0.0));
        assert_eq!(0.0, v.x());
        assert_eq!(0.0, v.y());
        assert_eq!(0.0, v.z());

        v = vorton.velocity_contribution(&Position::new(1.0, 0.0, 0.0));
        assert_eq!(0.0, v.x());
        assert_eq!(0.0, v.y());
        assert_eq!(0.0, v.z());

        v = vorton.velocity_contribution(&Position::new(0.0, 1.0, 0.0));
        assert_eq!(0.0, v.x());
        assert_eq!(0.0, v.y());
        assert_eq!(1.5*3.0/(4.0*std::f64::consts::PI), v.z());

        Ok(())
    }
}
*/
