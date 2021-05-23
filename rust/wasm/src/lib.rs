
use wasm_bindgen::{JsValue, prelude::*};
use web_sys;

use vortex_particle_simulation::{Simulation, Profiler};

static mut SIMULATION: Option<Simulation> = None;

#[wasm_bindgen(module = "/functions.js")]
extern "C" {
    fn time_now_ms() -> f64;
}


#[wasm_bindgen]
pub fn make_from_configuration(content: &str) -> Result<(), JsValue> {
    web_sys::console::log_1(&"make_from_configuration".into());
    // let content: Configuration = serde_wasm_bindgen::from_value(content)?;
    match Simulation::make_from_configuration(content.as_bytes()) {
        Ok(sim) => {
            web_sys::console::log_1(&JsValue::from_str(format!("Simulation constructed:").as_str()));
            web_sys::console::log_1(&JsValue::from_str(format!("Number of vortons: {}", sim.vortons().len()).as_str()));
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
                    web_sys::console::log_1(&JsValue::from_str(
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

