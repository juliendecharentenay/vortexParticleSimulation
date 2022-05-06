use std::{
    default::Default,
    future::Future,
    sync::{Arc, Mutex,},
};
use wasm_bindgen::{JsCast, JsValue, prelude::wasm_bindgen};
use js_sys::{ArrayBuffer, Uint8Array};
use web_sys::{console, MouseEvent, WheelEvent};

mod parameters;
use parameters::Parameters;

mod solution;
use solution::Solution;

mod viewer;
use viewer::Viewer;

mod camera;
use camera::Camera;

#[wasm_bindgen]
pub struct Simulation {
    parameters: Parameters,
    solution: Option<Solution>,
    viewer: Option<Arc<Mutex<Viewer>>>,
    camera: Option<Camera>,
}

#[wasm_bindgen]
impl Simulation {
    pub fn default() -> Self {
        Simulation { 
            parameters: Parameters::default(),
            solution: None,
            viewer: None,
            camera: None,
        }
    }
}

#[wasm_bindgen]
impl Simulation {
    pub fn set_parameters(&mut self, data: &str) -> Result<(), JsValue> {
        self.parameters = Parameters::from_json(data)
            .map_err(|e| JsValue::from_str(format!("Error in Simulation::set_parameters: {}", e).as_str()))?;
        Ok(())
    }

    pub fn get_parameters(&self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from_str(
                self.parameters.to_json()
                .map_err(|e| JsValue::from_str(format!("Error in Simulation::get_parameters: {}", e).as_str()))?
                .as_str()
                ))
    }
}

#[wasm_bindgen]
impl Simulation {
    pub fn initialize_solution(&mut self) -> Result<(), JsValue> {
        self.solution = Some( 
            Solution::from_configuration(
                serde_json::to_string(self.parameters.configuration())
                .map_err(|e| JsValue::from_str(format!("Error in Simulation::initialize_solution: {}", e).as_str()))?
                .as_str())
            .map_err(|e| JsValue::from_str(format!("Error in Simulation::initialize_solution: {}", e).as_str()))?);
        Ok(())
    }

    pub fn step(&mut self, time_step: f64) -> Result<(), JsValue> {
        self.solution.as_mut()
            .ok_or_else(|| JsValue::from_str(format!("Simulation::step - solution is not available").as_str()))?
            .step(time_step)
            .map_err(|e| JsValue::from_str(format!("Simulation::step - Error: {}", e).as_str()))?;
        Ok(())
    }

    pub fn iteration(&self) -> JsValue {
        if let Some(solution) = self.solution.as_ref() {
            JsValue::from_f64(solution.iteration() as f64)
        } else {
          JsValue::from_f64(0f64)
        }
    }

    pub fn time(&self) -> JsValue {
        if let Some(solution) = self.solution.as_ref() {
            JsValue::from_f64(solution.time() as f64)
        } else {
          JsValue::from_f64(0f64)
        }
    }

    pub fn solution_to_arraybuffer(&self) -> Result<ArrayBuffer, JsValue> {
        match &self.solution {
            None    => Err(JsValue::from_str(format!("Simulation::solution_to_arraybuffer: No solution is available to be converted.").as_str())),
            Some(s) => {
                Ok(s.to_arraybuffer()
                   .map_err(|e| JsValue::from_str(format!("Simulation::solution_to_arraybuffer - Error converting simulation to array buffer: {}", e).as_str()))?)
            },
        }
    }

    pub fn solution_from_arraybuffer(&mut self, data: ArrayBuffer) -> Result<(), JsValue> {
        self.solution = Some( Solution::from_arraybuffer(data)
                              .map_err(|e| JsValue::from_str(format!("Simulation::solution_from_arraybuffer - Error converting array buffer to simulation: {}", e).as_str()))? );
        Ok(())
    }
}

#[wasm_bindgen]
impl Simulation {
    pub fn initialize_viewer(&mut self, element_id: &str) -> Result<(), JsValue> {
        let viewer = Viewer::from_element_id(element_id)
            .map_err(|e| JsValue::from_str(format!("Simulation::initialize_viewer - Error making viewer: {}", e).as_str()))?;
        self.viewer = Some(Arc::new(Mutex::new(viewer)));
        Ok(())
    }

    pub fn create_view(&mut self, data: JsValue) -> js_sys::Promise {
        if self.viewer.is_none() {
            return js_sys::Promise::reject(&JsValue::from_str("Simulation::draw - Error: view is not initialised"));
        }
        let viewer = Arc::clone(self.viewer.as_ref().unwrap());

        let data = data.as_string();
        if data.is_none() {
            return js_sys::Promise::reject(&JsValue::from_str(format!("Simulation::create_view - Error: data {:?} can not be converted to string", data).as_str()));
        }
        let data = data.unwrap();

        wasm_bindgen_futures::future_to_promise(
          async move {
              viewer.lock().unwrap()
                  .create_view(data.as_str())
                  .await
                  .map(|uuid| JsValue::from_str(uuid.to_hyphenated().to_string().as_str()))
                  .map_err(|e| JsValue::from_str(format!("{}", e).as_str()))
          }
        )
    }

    pub fn draw(&mut self) -> Result<(), JsValue> {
        if self.camera.is_none() {
            self.camera 
                = Some(Camera::new()
                       .map_err(|e| JsValue::from_str(format!("Simulation::draw - Error creating camera: {:?}", e).as_str()))?);
        }
        let mut camera = self.camera.as_mut().unwrap();
        let mut viewer = self.viewer.as_ref()
            .ok_or_else(|| JsValue::from_str("Simulation::draw - Error: viewer is not initialised"))?
            .lock().unwrap();

        if camera.width().is_none() 
            || camera.height().is_none() 
            || (camera.width().unwrap() - viewer.width() as f32).abs() > 1.0 
            || (camera.height().unwrap() - viewer.height() as f32).abs() > 1.0 {
                camera.set(viewer.width() as f32, viewer.height() as f32)
                    .map_err(|e| JsValue::from_str(format!("Simulation::draw - Error setting camera width/height: {:?}", e).as_str()))?;
            }

        if let Some(solution) = self.solution.as_ref() {
            viewer.draw(solution, &camera)
                .map_err(|e| JsValue::from_str(format!("Simulation::draw - Error: {}", e).as_str()))?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Simulation {
    pub fn on_mouse_down(&mut self, event: JsValue) -> Result<(), JsValue> {
        if let Some(camera) = self.camera.as_mut() {
            let event: MouseEvent = event.dyn_into()?;
            camera.on_mouse_down(event)
                .map_err(|e| JsValue::from_str(format!("Simulation::on_mouse_down - Error: {}", e).as_str()))?;
        }
        Ok(())
    }

    pub fn on_mouse_move(&mut self, event: JsValue) -> Result<(), JsValue> {
        if let Some(camera) = self.camera.as_mut() {
            let event: MouseEvent = event.dyn_into()?;
            camera.on_mouse_move(event)
                .map_err(|e| JsValue::from_str(format!("Simulation::on_mouse_move - Error: {}", e).as_str()))?;
        }
        Ok(())
    }

    pub fn on_mouse_up(&mut self, event: JsValue) -> Result<(), JsValue> {
        if let Some(camera) = self.camera.as_mut() {
            let event: MouseEvent = event.dyn_into()?;
            camera.on_mouse_up(event)
                .map_err(|e| JsValue::from_str(format!("Simulation::on_mouse_up - Error: {}", e).as_str()))?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Simulation {
    pub fn on_wheel(&mut self, event: JsValue) -> Result<(), JsValue> {
        if let Some(camera) = self.camera.as_mut() {
            camera.on_wheel(event.dyn_into()?)
                .map_err(|e| JsValue::from_str(format!("Simulation::on_wheel - Error: {}", e).as_str()))?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Simulation {
    pub fn on_touch_event(&mut self, event: JsValue) -> Result<(), JsValue> {
        if let Some(camera) = self.camera.as_mut() {
            camera.on_touch_event(event.dyn_into()?)
                .map_err(|e| JsValue::from_str(format!("Simulation::on_touch_event - Error: {}", e).as_str()))?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl Simulation {
    pub fn on_controller_signal(&mut self, signal: JsValue) -> Result<(), JsValue> {
        if let Some(camera) = self.camera.as_mut() {
            camera.on_controller_signal(signal.into_serde()
                                        .map_err(|e| JsValue::from_str(format!("Simulation::on_controller_signal - Error: {}", e).as_str()))?)
                .map_err(|e| JsValue::from_str(format!("Simulation::on_controller_signal - Error: {}", e).as_str()))?;
        }
        Ok(())
    }
}
