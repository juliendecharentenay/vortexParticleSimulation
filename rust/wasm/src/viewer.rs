use std::error::Error;

use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext};
use nalgebra::Matrix4;

use vortex_particle_simulation::{Simulation};

use crate::{solver::Solver, camera::CameraBuilder};

mod main;
use main::Main;

mod webgl;
pub use webgl::{webgl_link_program, webgl_compile_vertex_shader, webgl_compile_fragment_shader, webgl_compile_shader};

mod program_demo;
mod program_vorton_render;

pub trait ViewerElement {
    fn draw(&mut self, context: &WebGl2RenderingContext, camera: &Matrix4<f32>, simulation: &Simulation) -> Result<(), Box<dyn Error>>;
    fn redraw(&mut self, context: &WebGl2RenderingContext, camera: &Matrix4<f32>) -> Result<(), Box<dyn Error>>;
}

#[wasm_bindgen]
pub struct Viewer {
    viewer: Main,
    simulation: Option<Simulation>,
}

#[wasm_bindgen]
impl Viewer {
    pub fn new(element_id: &str) -> Result<Viewer, JsValue> {
        match Main::new(element_id) {
            Ok(v) => Ok(Viewer { viewer: v, simulation: None}),
            Err(e) => Err(JsValue::from_str(
                format!("Unable to create viewer. Error: {:?}", e).as_str()
                )),
        }
    }

    pub fn draw(&mut self, camera_builder: &CameraBuilder) -> Result<(), JsValue> {
        if self.simulation.is_some() {
           self.viewer.draw(
               self.simulation.as_ref().unwrap(),
               &camera_builder.to_matrix4().map_err(|e| JsValue::from_str(e.to_string().as_str()))?
           ).map_err(|e| JsValue::from_str(e.to_string().as_str()))?;
        }
        Ok(())
    }

    pub fn set_simulation(&mut self, solver: Solver) -> Result<(), JsValue> {
        self.simulation = Some(solver.into_simulation());
        Ok(())
    }

    pub fn create(&mut self, content: &str) -> Result<JsValue, JsValue> {
        match self.viewer.create(content) {
            Ok(uuid) => Ok(JsValue::from_str(format!("{}", uuid.to_hyphenated()).as_str())),
            Err(e) => Err(JsValue::from_str(format!("Unable to create element from {}. Error: {}", content, e).as_str())),
        }
    }
}

