use serde::{Serialize, Deserialize};

use nalgebra::{Point3, Vector3};

use crate::configuration::{InitialConditions};

#[derive(Serialize, Deserialize)]
pub struct VortexRing {
    pub center: Point3<f64>,
    pub direction: Vector3<f64>,
    pub intensity: f64,
    pub radius: f64,
    pub thickness: f64,
}

impl VortexRing {
    fn direction_normalized(&self) -> Vector3<f64> {
        if self.direction.norm() < 1e-5 {
            Vector3::x()
        } else {
            self.direction.normalize()
        }
    }
}

impl InitialConditions for VortexRing {
    fn domain(&self) -> (Point3<f64>, Point3<f64>) {
        let dir = self.direction_normalized();
        let dcx = Vector3::x().cross(&dir);
        let v2 = if dcx.norm() > 1e-5 { dcx } else { Vector3::y() };
        let v3 = dir.cross(&v2);
        let min = self.center
            - dir.scale(1.1*self.thickness)
            - v2.scale(1.1*(self.radius+self.thickness))
            - v3.scale(1.1*(self.radius+self.thickness));
        let max = self.center
            + dir.scale(1.1*self.thickness)
            + v2.scale(1.1*(self.radius+self.thickness))
            + v3.scale(1.1*(self.radius+self.thickness));
        (min, max)
    }

    fn vorticity(&self, p: &Point3<f64>) -> Vector3<f64> {
        let dir = self.direction_normalized();
        let v = p - self.center;         // Vector [center of vortex ring to point]
        let d_dir = v.dot(&dir);         // Distance in the vortex ring direction
        let v_rad = v - dir.scale(d_dir); // Vector in the radial direction
        let d_rad = v_rad.norm() - self.radius; // Distance from the ring center in the radial plane
        let d = (d_dir.powi(2) + d_rad.powi(2)).sqrt(); // Distance from the ring center
        if d < self.thickness {
            let vorticity_magnitude = self.intensity * (0.5 + 0.5*(std::f64::consts::PI * d / self.thickness).cos());
            dir.cross(&v_rad).normalize().scale(vorticity_magnitude)
        } else { 
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
}
