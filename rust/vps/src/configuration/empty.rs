use serde::{Serialize, Deserialize};

use crate::algebra::{Point3, Vector3};
use crate::configuration::{InitialConditions};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Empty {
}

impl Empty {
    pub fn new() -> Empty {
        Empty { }
    }
}

impl InitialConditions for Empty {
    fn free_stream_velocity(&self) -> Vector3<f64> {
        Vector3::<f64>::new(2.0, 0.0, 0.0)
    }

    fn domain(&self) -> (Point3<f64>, Point3<f64>) {
        (Point3::<f64>::new(-10.0, -10.0, -10.0), Point3::<f64>::new(10.0, 10.0, 10.0))
    }

    fn vorticity(&self, _p: &Point3<f64>) -> Vector3<f64> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}
