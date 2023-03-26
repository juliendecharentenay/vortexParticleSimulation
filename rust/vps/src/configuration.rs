use serde::{Serialize, Deserialize};

use crate::algebra::{Point3, Vector3};

pub mod vortexring;
pub use vortexring::VortexRing;
pub mod empty;

pub trait InitialConditions {
    fn free_stream_velocity(&self) -> Vector3<f64>;
    fn domain(&self) -> (Point3<f64>, Point3<f64>);
    fn vorticity(&self, p: &Point3<f64>) -> Vector3<f64>;
}

#[derive(Serialize, Deserialize, Clone)]
pub enum InitialConditionData {
    InitialConditionVortexRing(VortexRing),
    InitialConditionEmpty(empty::Empty),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Domain {
    pub min: Point3<f64>,
    pub max: Point3<f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub n_vortons: usize,
    pub initial_conditions: InitialConditionData,
    pub domain: Domain,
    pub viscosity: f64,
}

impl Configuration {
    pub fn new_vortex_ring() -> Configuration {
        Configuration {
            n_vortons: 1000,
            initial_conditions: InitialConditionData::InitialConditionVortexRing(
                VortexRing {
                    center: Point3::<f64>::new(0.0, 0.0, 0.0),
                    direction: Vector3::<f64>::new(1.0, 0.0, 0.0),
                    intensity: 1.0,
                    radius: 1.0,
                    thickness: 0.5,
                }
                ),
            domain: Domain { min: Point3::<f64>::new(0.0, 0.0, 0.0), max: Point3::<f64>::new(1.0, 1.0, 1.0) },
            viscosity: 1e-5
        }
    }

    pub fn new() -> Configuration {
        Configuration {
            n_vortons: 0,
            initial_conditions: InitialConditionData::InitialConditionEmpty(empty::Empty::new()),
            domain: Domain { min: Point3::<f64>::new(0.0, 0.0, 0.0), max: Point3::<f64>::new(1.0, 1.0, 1.0) },
            viscosity: 1e-5
        }
    }

    pub fn get_initial_conditions(&self) -> Box<& dyn InitialConditions> {
        match &self.initial_conditions {
            InitialConditionData::InitialConditionVortexRing(v) => Box::new(v),
            InitialConditionData::InitialConditionEmpty(v) => Box::new(v),
        }
    }
}
