/*
 * Main class
 */
use std::{
    error::Error,
    convert::TryInto,
};
use std::collections::HashMap;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};

use simple_error::SimpleError;
// use nanoid::nanoid;
use uuid::Uuid;
use nalgebra::Matrix4;

use serde::{Deserialize};
use serde_json;

use vortex_particle_simulation::{Simulation};

use crate::viewer::{ViewerElement, webgl_link_program, webgl_compile_shader};
use crate::viewer::program_demo::ProgramDemo;
use crate::viewer::program_vorton_render::ProgramVortonRender;

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ViewerElementType {
    Demo,
    VortonRender,
}

pub struct Main 
{
    canvas: HtmlCanvasElement,
    // context: WebGl2RenderingContext,
    viewer_elements: HashMap<Uuid, Box<dyn ViewerElement>>,
}

impl Main
{
    pub fn context(&self) -> Result<WebGl2RenderingContext, Box<dyn Error>> {
        let canvas = &self.canvas;
        let context 
            = match canvas.get_context("webgl2")
            {
                Ok(c) => c,
                Err(_) => return Err(Box::new(SimpleError::new(format!("Unable to retrieve webgl2 context from canvas").as_str()))),
            };
        let context 
            = match context.unwrap().dyn_into::<WebGl2RenderingContext>()
            {
                Ok(c) => c,
                Err(_) => return Err(Box::new(SimpleError::new(format!("Unable to cast webgl2 context appropriately from canvas").as_str()))),
            };
        Ok(context)
    }

    /*
    pub fn get_context(& self) -> &WebGl2RenderingContext { &self.context }
    pub fn get_context_mut(&mut self) -> &mut WebGl2RenderingContext { &mut self.context }
    */
}

impl Main
{
    /*
     * Build viewer based on canvas dom element_id
     */
    pub fn new(element_id: &str) -> Result<Main, Box<dyn Error>> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(element_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement 
            = match canvas.dyn_into::<web_sys::HtmlCanvasElement>() 
            {
                Ok(c) => c,
                Err(_) => return Err(Box::new(SimpleError::new(format!("Unable to cast HTML element {} into canvas element", element_id).as_str()))),
            };
        Ok(Main { 
               canvas,
               viewer_elements: HashMap::new(),
        })
    }

    pub fn reset(&mut self) -> Result<(), Box<dyn Error>> {
        for e in self.viewer_elements.values_mut() { e.reset()?; }
        Ok(())
    }

    /*
     * Draw simulation
     */
    pub fn draw(&mut self, simulation: &Simulation, camera: &Matrix4<f32>) -> Result<(), Box<dyn Error>> {
        // Initialise background
        let mut context = self.context()?;
        context.viewport(0, 0, self.canvas.width().try_into()?, self.canvas.height().try_into()?);
        context.clear_color(6f32/255f32, 78f32/255f32, 59f32/255f32, 1f32);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        for e in self.viewer_elements.values_mut() {
            e.draw(&context, &camera, &simulation)?;
        }

        Ok(())
    }

    /*
     * Helper function to insert a viewer element
     */
    fn insert_viewer_element<T>(&mut self, v: T) -> Uuid 
        where T: 'static + ViewerElement
    {
        let id = Uuid::new_v4();
        self.viewer_elements.insert(id.clone(), Box::new(v));
        id
    }

    /*
     * Create and store a viewer element
     */
    pub fn create(&mut self, content: &str) -> Result<Uuid, Box<dyn Error>>
    {
        match serde_json::from_str(content) {
            Ok(t) => {
                let id = match t {
                    ViewerElementType::Demo => self.insert_viewer_element(ProgramDemo::new(&self.context()?)?),
                    ViewerElementType::VortonRender => self.insert_viewer_element(ProgramVortonRender::new(&self.context()?)?),
                };
                Ok(id)
            },
            Err(e) => Err(Box::new(SimpleError::new(format!("Unable to identify element to created from string {}. Error: {}", content, e).as_str()))),
        }
    }

}
