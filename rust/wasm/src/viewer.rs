use std::error::Error;

use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext};

use vortex_particle_simulation::{Simulation};

use crate::{to_vpm_simulation};

mod main;
use main::Main;

mod webgl;
pub use webgl::{webgl_link_program, webgl_compile_vertex_shader, webgl_compile_fragment_shader, webgl_compile_shader};

mod camera;
use camera::Camera;
mod program_demo;
mod program_vorton_render;

static mut VIEWER: Option<Main> = None;

pub trait ViewerElement {
    fn draw(&mut self, context: &WebGl2RenderingContext, camera: &Camera, simulation: &Simulation) -> Result<(), Box<dyn Error>>;
    fn redraw(&mut self, context: &WebGl2RenderingContext, camera: &Camera) -> Result<(), Box<dyn Error>>;
}

#[wasm_bindgen]
pub fn viewer_start(element_id: &str) -> Result<(), JsValue> {
    match Main::new(element_id) {
        Ok(v) => {
            unsafe { VIEWER = Some(v); }
            Ok(())
        },
        Err(e) => Err(JsValue::from_str(
                format!("Unable to create viewer. Error: {:?}", e).as_str()
                )),
    }
}

fn on_viewer<T, F>(f: F) -> Result<T, JsValue> 
where F: Fn(& Main) -> Result<T, JsValue>
{
    match unsafe { &VIEWER } {
        Some(v) => f(v),
        None => Err(JsValue::from_str("Viewer is not initialized")),
    }
}

fn on_viewer_mut<T, F>(f: F) -> Result<T, JsValue> 
where F: FnOnce(&mut Main) -> Result<T, JsValue>
{
    match unsafe { &mut VIEWER } {
        Some(v) => f(v),
        None => Err(JsValue::from_str("Viewer is not initialized")),
    }
}

#[wasm_bindgen]
pub fn viewer_draw(sim: JsValue) -> Result<(), JsValue> {
    on_viewer_mut(|viewer| {
      match to_vpm_simulation(sim) {
        Ok(s) => {
          match viewer.draw(&s) {
            Ok(_) => Ok(()),
            Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
          }
        },
        Err(e) => Err(JsValue::from_str(e.to_string().as_str())),
      }
    })
}

#[wasm_bindgen]
pub fn viewer_element_create(content: &str) -> Result<JsValue, JsValue> {
    on_viewer_mut(|viewer| {
      match viewer.create(content) {
        Ok(uuid) => Ok(JsValue::from_str(format!("{}", uuid.to_hyphenated()).as_str())),
        Err(e) => Err(JsValue::from_str(format!("Unable to create element from {}. Error: {}", content, e).as_str())),
      }
    })
}

#[wasm_bindgen]
pub fn viewer_element_read(id: &str) -> Result<JsValue, JsValue> {
    Err(JsValue::from_str("viewer_element_read is not implemented"))
}

#[wasm_bindgen]
pub fn viewer_element_update(id: &str, content: &str) -> Result<JsValue, JsValue> {
    Err(JsValue::from_str("viewer_element_updated is not implemented"))
}

#[wasm_bindgen]
pub fn viewer_element_delete(id: &str) -> Result<JsValue, JsValue> {
    Err(JsValue::from_str("viewer_element_delete is not implemented"))
}

