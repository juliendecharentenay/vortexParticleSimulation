use std::error::Error;

use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext};

use vortex_particle_simulation::{Simulation};

use crate::{solver::Solver};

mod main;
use main::Main;

mod webgl;
pub use webgl::{webgl_link_program, webgl_compile_vertex_shader, webgl_compile_fragment_shader, webgl_compile_shader};

mod camera;
use camera::Camera;
mod program_demo;
mod program_vorton_render;

pub trait ViewerElement {
    fn draw(&mut self, context: &WebGl2RenderingContext, camera: &Camera, simulation: &Simulation) -> Result<(), Box<dyn Error>>;
    fn redraw(&mut self, context: &WebGl2RenderingContext, camera: &Camera) -> Result<(), Box<dyn Error>>;
}

#[wasm_bindgen]
pub struct Viewer {
    viewer: Main,
}

#[wasm_bindgen]
impl Viewer {
    pub fn new(element_id: &str) -> Result<Viewer, JsValue> {
        match Main::new(element_id) {
            Ok(v) => Ok(Viewer { viewer: v}),
            Err(e) => Err(JsValue::from_str(
                format!("Unable to create viewer. Error: {:?}", e).as_str()
                )),
        }
    }

    pub fn draw(&mut self, solver: Solver) -> Result<(), JsValue> {
        match self.viewer.draw(solver.get_simulation()) {
            Ok(_) => Ok(()),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
        }
    }

    pub fn create(&mut self, content: &str) -> Result<JsValue, JsValue> {
        match self.viewer.create(content) {
            Ok(uuid) => Ok(JsValue::from_str(format!("{}", uuid.to_hyphenated()).as_str())),
            Err(e) => Err(JsValue::from_str(format!("Unable to create element from {}. Error: {}", content, e).as_str())),
        }
    }
}

