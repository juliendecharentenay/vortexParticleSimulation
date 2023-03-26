use std::{
    error::Error,
};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use web_sys::{console};
use js_sys::{ArrayBuffer, Uint8Array};
use bincode;

use vortex_particle_simulation::{Simulation, Profiler};

#[wasm_bindgen(module = "/functions.js")]
extern "C" {
    fn time_now_ms() -> f64;
}

pub struct Solution {
    simulation: Simulation,
}

impl Solution {
    pub fn simulation_ref(&self) -> &Simulation { &self.simulation }
}

impl Solution {
    pub fn from_configuration(configuration: &str) -> Result<Solution, Box<dyn Error>> {
        Ok( Solution { simulation: Simulation::make_from_configuration(serde_json::from_str(configuration)?)? })
    }

    pub fn from_arraybuffer(data: ArrayBuffer) -> Result<Solution, Box<dyn Error>> {
        let a = Uint8Array::new(&data);
        Ok( Solution { simulation: bincode::deserialize(&a.to_vec()[..])? })
    }

    pub fn to_arraybuffer(&self) -> Result<ArrayBuffer, Box<dyn Error>> {
        let b = bincode::serialize(&self.simulation)?;
        Ok(Uint8Array::from(&b[..]).buffer())
    }

    pub fn iteration(&self) -> usize {
      self.simulation.iteration()
    }

    pub fn time(&self) -> f64 {
        self.simulation.time()
    }

    pub fn step(&mut self, time_step: f64) -> Result<(), Box<dyn Error>> {
        let mut profiler = Profiler::new(|| {time_now_ms()}).unwrap();

         match self.simulation.step(time_step, &mut profiler) {
                Ok(_) => {
                    console::log_1(&JsValue::from_str(
                            format!("Iteration {} - {:.2}s [{}]", self.simulation.iteration(), self.simulation.time(),
                                    profiler.as_magnitude()
                                    .iter().fold("".to_string(), |r, v| format!("{}{}{}: {:.1}ms", r, if r.is_empty() {""} else {"; "}, v.0, v.1))
                                    ).as_str()));
                    Ok(())
                },
                Err(e) => Err(e),
            }
    }
}

