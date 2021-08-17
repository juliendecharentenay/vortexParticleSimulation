
use wasm_bindgen::{JsValue, prelude::*};
use web_sys::{console};

use vortex_particle_simulation::{Simulation, Profiler};

use crate::{from_vpm_simulation};

static mut SIMULATION: Option<Simulation> = None;

#[wasm_bindgen(module = "/functions.js")]
extern "C" {
    fn time_now_ms() -> f64;
}


#[wasm_bindgen]
pub fn make_from_configuration(content: &str) -> Result<(), JsValue> {
    console::log_1(&"make_from_configuration".into());
    // let content: Configuration = serde_wasm_bindgen::from_value(content)?;
    match Simulation::make_from_configuration(content.as_bytes()) {
        Ok(sim) => {
            console::log_1(&JsValue::from_str(format!("Simulation constructed:").as_str()));
            console::log_1(&JsValue::from_str(format!("Number of vortons: {}", sim.vortons().len()).as_str()));
            unsafe { 
                SIMULATION = Some(sim); 
            };
            Ok(())
        },
        Err(e)  => Err(JsValue::from_str(format!("{}", e).as_str())),
    }
}

#[wasm_bindgen]
pub fn step(time_step: f64) -> Result<(), JsValue> {
    let mut profiler = Profiler::new(|| {time_now_ms()}).unwrap();
    match unsafe { &mut SIMULATION } {
        Some(sim) => 
            match sim.step(time_step, &mut profiler) {
                Ok(_) => {
                    console::log_1(&JsValue::from_str(
                            format!("Iteration {} - {:.2}s [{}]", sim.iteration(), sim.time(),
                                    profiler.as_magnitude()
                                    .iter().fold("".to_string(), |r, v| format!("{}{}{}: {:.1}ms", r, if r.is_empty() {""} else {"; "}, v.0, v.1))
                                    ).as_str()));
                    Ok(())
                },
                Err(e) => Err(JsValue::from_str(format!("{}", e).as_str())),
            },
        None => Err(JsValue::from_str("Simulation is not initialized")),
    }
}

#[wasm_bindgen]
pub fn iteration() -> JsValue {
    match unsafe { &SIMULATION } {
        Some(sim) => JsValue::from_f64(sim.iteration() as f64),
        None => JsValue::from_f64(0f64),
    }
}

#[wasm_bindgen]
pub fn time() -> JsValue {
    match unsafe { &SIMULATION } {
        Some(sim) => JsValue::from_f64(sim.time()),
        None => JsValue::from_f64(0f64),
    }
}

#[wasm_bindgen]
pub fn get_simulation() -> JsValue {
    match unsafe { &SIMULATION } {
        Some(sim) => from_vpm_simulation(sim),
        None => from_vpm_simulation(&Simulation::new()),
    }
}

pub fn get_simulation_deprec() -> Simulation {
    console::log_1(&JsValue::from_str("solver::get_simulation"));
    match unsafe { &SIMULATION } {
        Some(sim) => {
            console::log_1(&JsValue::from_str("Simulaiton exists... clone"));
            sim.clone()
        },
        None => {
            console::log_1(&JsValue::from_str("Simulaiton DOES NOT exists... make a new one"));
            Simulation::new()
        },
    }
}

