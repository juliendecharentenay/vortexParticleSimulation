use std::error::Error;

use wasm_bindgen::JsValue;
use simple_error::SimpleError;

mod viewer;
mod solver;

pub fn from_vpm_simulation(sim: &vortex_particle_simulation::Simulation) -> JsValue {
    JsValue::from_serde(sim).unwrap()
}

pub fn to_vpm_simulation(js_value: JsValue) -> Result<vortex_particle_simulation::Simulation, Box<dyn Error>> {
    match JsValue::into_serde(&js_value) {
        Ok(sim) => Ok(sim),
        Err(e) => Err(Box::new(SimpleError::new(format!("Unable to parse to simulation. Error {}", e).as_str()))),
    }
}

