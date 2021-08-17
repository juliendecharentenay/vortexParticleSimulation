use std::error::Error;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::algebra::{Point3, Vector3};

pub mod vortexring;
pub use vortexring::VortexRing;
pub mod empty;

pub trait InitialConditions {
    fn free_stream_velocity(&self) -> Vector3<f64>;
    fn domain(&self) -> (Point3<f64>, Point3<f64>);
    fn vorticity(&self, p: &Point3<f64>) -> Vector3<f64>;
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum InitialConditionData {
    InitialConditionVortexRing(VortexRing),
    InitialConditionEmpty(empty::Empty),
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Domain {
    pub min: Point3<f64>,
    pub max: Point3<f64>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Configuration {
    pub n_vortons: usize,
    pub initial_conditions: InitialConditionData,
    pub domain: Domain,
    pub viscosity: f64,
}

impl Configuration {
    /*
    pub fn make_from_json_file(filename: &String) -> Result<Configuration, Box<dyn Error>> {
        let f = File::open(filename)?;
        let r = BufReader::new(f);
        Ok(serde_json::from_reader(r)?)
    }
    */
    pub fn new() -> Configuration {
        Configuration {
            n_vortons: 0,
            initial_conditions: InitialConditionData::InitialConditionEmpty(empty::Empty::new()),
            domain: Domain { min: Point3::<f64>::new(0.0, 0.0, 0.0), max: Point3::<f64>::new(1.0, 1.0, 1.0) },
            viscosity: 1e-5
        }
    }

    pub fn make_from(content: &[u8]) -> Result<Configuration, Box<dyn Error>> {
        Ok(serde_json::from_slice(content)?)
    }

    pub fn get_initial_conditions(&self) -> Box<& dyn InitialConditions> {
        match &self.initial_conditions {
            InitialConditionData::InitialConditionVortexRing(v) => Box::new(v),
            InitialConditionData::InitialConditionEmpty(v) => Box::new(v),
        }
    }
}
