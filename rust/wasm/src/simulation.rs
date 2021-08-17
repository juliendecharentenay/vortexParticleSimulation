use std::error::Error;

use wasm_bindgen::prelude::*;
use serde_json;
use simple_error;
use js_sys::JsString;

use vortex_particle_simulation;

#[wasm_bindgen]
pub struct Simulation {
    pub data: JsValue,
}


impl Simulation {
    pub fn from_vpm_simulation(sim: &vortex_particle_simulation::Simulation) -> Simulation {
        Simulation {
            data: JsValue::from_str(serde_json::to_string(sim).as_str()),
        }
    }

    pub fn to_vpm_simulation(&self) -> Result<vortex_particle_simulation::Simulation, Box<dyn Error>> {
        match self.data.as_string() {
            Some(d) => {
                match serde_json::from_str(d.as_str()) { 
                    Ok(sim) => sim,
                    Err(e) => Err(Box::new(SimpleError::new("Unable to parse to simulation. Error {}", e))),
                }
            },
            None => Err(Box::new(SimpleError::new("Unable to recover data"))),
        }
    }
}

