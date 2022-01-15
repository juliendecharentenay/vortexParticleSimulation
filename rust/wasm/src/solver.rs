
use wasm_bindgen::{JsValue, prelude::*};
use web_sys::{console};
use bincode;
use js_sys::{SharedArrayBuffer, ArrayBuffer, Uint8Array};

use vortex_particle_simulation::{Simulation, Profiler};

#[wasm_bindgen(module = "/functions.js")]
extern "C" {
    fn time_now_ms() -> f64;
}

#[wasm_bindgen]
pub struct Solver {
    simulation: Simulation,
}

#[wasm_bindgen]
impl Solver {
    pub fn from_configuration(content: &str) -> Result<Solver, JsValue> {
      match Simulation::make_from_configuration(content.as_bytes()) {
        Ok(sim) => Ok(Solver { simulation: sim}),
        Err(e)  => Err(JsValue::from_str(format!("{}", e).as_str())),
      }
    }

    pub fn from_json(content: JsValue) -> Result<Solver, JsValue> {
        match JsValue::into_serde(&content) {
            Ok(sim) => Ok(Solver { simulation: sim} ),
            Err(e) => Err(JsValue::from_str(format!("Unable to parse to simulation. Error {}", e).as_str())),
        }
    }

    pub fn from_array_buffer(content: ArrayBuffer) -> Result<Solver, JsValue> {
        let a = Uint8Array::new(&content);
        match bincode::deserialize(&a.to_vec()[..]) {
            Ok(sim) => Ok(Solver { simulation: sim } ),
            Err(e) => Err(JsValue::from_str(format!("Unable to retrieve simulation from ArrayBuffer. Error {}", e).as_str())),
        }
    }

    pub fn from_shared_array_buffer(content: SharedArrayBuffer) -> Result<Solver, JsValue> {
        let a = Uint8Array::new(&content);
        match bincode::deserialize(&a.to_vec()[..]) {
            Ok(sim) => Ok(Solver { simulation: sim } ),
            Err(e) => Err(JsValue::from_str(format!("Unable to retrieve simulation from SharedArrayBuffer. Error {}", e).as_str())),
        }
    }

    pub fn iteration(&self) -> JsValue {
      JsValue::from_f64(self.simulation.iteration() as f64)
    }

    pub fn time(&self) -> JsValue {
        JsValue::from_f64(self.simulation.time())
    }

    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(&self.simulation).unwrap()
    }

    pub fn to_array_buffer(&self) -> ArrayBuffer {
        let b = bincode::serialize(&self.simulation).unwrap();
        Uint8Array::from(&b[..]).buffer()
    }

    pub fn to_shared_array_buffer(&self) -> SharedArrayBuffer {
        let b = bincode::serialize(&self.simulation).unwrap();
        let mut r = SharedArrayBuffer::new(b.len() as u32);
        let mut a = Uint8Array::new(&r);
        for i in 0..b.len() { a.set_index(i as u32, b[i]); }
        r
    }

    pub fn step(&mut self, time_step: f64) -> Result<(), JsValue> {
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
                Err(e) => Err(JsValue::from_str(format!("{}", e).as_str())),
            }
    }
}

impl Solver {
    pub fn get_simulation(&self) -> &Simulation {
        &self.simulation
    }
}

