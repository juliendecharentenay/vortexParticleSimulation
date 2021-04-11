use serde::{Serialize, Deserialize};
use nalgebra::{Point3, Vector3};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vorton {
    radius: f64,
    volume: f64,
    position: Point3<f64>,
    vorticity: Vector3<f64>,
}

impl Vorton {
    pub fn new(position: Point3<f64>, vorticity: Vector3<f64>, volume: f64) -> Vorton {
        Vorton {
            radius: volume.cbrt()*0.1,
            volume,
            position,
            vorticity,
        }
    }

    pub fn velocity_contribution(&self, position: &Point3<f64>) -> Vector3<f64> {
        let r = position - self.position;
        self.vorticity
            .cross(&r)
            .scale(1.0/(4.0 * std::f64::consts::PI * r.norm().max(self.radius).powi(3)) * self.volume)
    }

    pub fn advect(&self, velocity: &Vector3<f64>, time_step: f64) -> Vorton {
        Vorton::new(
            self.position + velocity.scale(time_step),
            self.vorticity.clone(),
            self.volume
        )
    }

    pub fn step(&self, source: &Vector3<f64>, time_step: f64) -> Vorton {
        Vorton::new(
            self.position.clone(),
            self.vorticity + source.scale(time_step),
            self.volume
        )
    }

    pub fn volume(&self)    -> f64      { self.volume.clone() }
    pub fn vorticity(&self) -> &Vector3<f64>  { &self.vorticity }
    pub fn position(&self)  -> &Point3<f64> { &self.position }
}

/*
impl Aggregatable for Vorton {
    fn aggregate(&self, other: &Vorton) -> Vorton {
        // let radius = 0.5*(self.radius + other.radius);
        let volume = self.volume + other.volume;
        let self_vort_vol = self.vorticity.norm()*self.volume;
        let other_vort_vol = other.vorticity.norm()*other.volume;
        let position = (self.position.map(|e| e*self_vort_vol)
            + (other.position.map(|e| e*other_vort_vol) - Point3::<f64>::origin()))
            .map(|e| e * 1.0 / (self_vort_vol + other_vort_vol));
        let vorticity = (self.vorticity.scale(self.volume)
            + other.vorticity.scale(other.volume))
            .scale(1.0/volume);
        Vorton::new(
            position,
            vorticity,
            volume
        )
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
