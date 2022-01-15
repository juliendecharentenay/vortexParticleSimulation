/*
 * Main class
 */
use std::error::Error;
use std::collections::HashMap;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, WebGl2RenderingContext, WebGlProgram, WebGlShader};

use simple_error::SimpleError;
// use nanoid::nanoid;
use uuid::Uuid;
use nalgebra::{Point3, Vector3};

use serde::{Deserialize};
use serde_json;

use vortex_particle_simulation::{Simulation};

use crate::viewer::{ViewerElement, webgl_link_program, webgl_compile_shader};
use crate::viewer::program_demo::ProgramDemo;
use crate::viewer::program_vorton_render::ProgramVortonRender;
use crate::viewer::camera::Camera;

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ViewerElementType {
    Demo,
    VortonRender,
}

/*
 * Trait to manipulate a camera
 */
pub trait CameraManipulator {
    fn apply(&mut self, camera: Camera) -> Camera;
}

pub struct Main 
{
    context: WebGl2RenderingContext,
    viewer_elements: HashMap<Uuid, Box<dyn ViewerElement>>,
    camera: Option<Camera>,
    camera_manipulator: Option<Box<dyn CameraManipulator>>,
}

impl Main
{
    pub fn get_context(& self) -> &WebGl2RenderingContext { &self.context }
    pub fn get_context_mut(&mut self) -> &mut WebGl2RenderingContext { &mut self.context }
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

        let context 
            = match canvas.get_context("webgl2")
            {
                Ok(c) => c,
                Err(_) => return Err(Box::new(SimpleError::new(format!("Unable to retrieve webgl2 context from canvas {}", element_id).as_str()))),
            };
        let context 
            = match context.unwrap().dyn_into::<WebGl2RenderingContext>()
            {
                Ok(c) => c,
                Err(_) => return Err(Box::new(SimpleError::new(format!("Unable to cast webgl2 context appropriately from canvas {}", element_id).as_str()))),
            };

        let mut m = Main { 
               context,
               viewer_elements: HashMap::new(),
               camera: None,
               camera_manipulator: None,
        };
        m.register_events(canvas)?;
        Ok(m)
    }

    fn register_events(&mut self, canvas: web_sys::HtmlCanvasElement) -> Result<(), Box<dyn Error>> {
        {
            let closure = Closure::wrap(Box::new(|event: web_sys::MouseEvent| {
                console::log_1(&JsValue::from_str(format!("onmousedown event - {:?}", event).as_str()));
            }) as Box<dyn FnMut(_)>);
            match canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()) {
                Ok(_) => (),
                Err(e) => return Err(Box::new(SimpleError::new(format!("Error when adding mousedown event listener. Err: {:?}", e.as_string()).as_str()))),
            };
            closure.forget();
        }
        Ok(())
    }

    /*
     * Draw simulation
     */
    pub fn draw(&mut self, simulation: &Simulation) -> Result<(), Box<dyn Error>> {
        // Initialise camera [if required]
        self.init_camera(simulation)?;

        // Initialise background
        self.context.clear_color(6f32/255f32, 78f32/255f32, 59f32/255f32, 1f32);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        for e in self.viewer_elements.values_mut() {
            e.draw(&self.context, self.camera.as_ref().unwrap(), &simulation)?;
        }

        Ok(())
    }

    /*
     * Initialise the camera if not initialised previously
     */
    fn init_camera(&mut self, simulation: &Simulation) -> Result<(), Box<dyn Error>> {
        match &self.camera {
            Some(c) => Ok(()),
            None => {
                let canvas = self.context.canvas().unwrap();
                let canvas: web_sys::HtmlCanvasElement 
                    = match canvas.dyn_into::<web_sys::HtmlCanvasElement>() { 
                        Ok(c) => c, 
                        Err(_) => return Err(Box::new(SimpleError::new(format!("Unable to cast HTML element into canvas element").as_str()))), 
                    };

                self.camera 
                    = Some(Camera::new(canvas.width(), canvas.height())
                           .set_target(Point3::new(0.0, 0.0, 0.0))?
                           .set_eye(Point3::new(0.0, 5.0, 0.0))?
                           .set_up(Vector3::new(0.0, 0.0, 1.0))?);
                Ok(())
            }
        }
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
                    ViewerElementType::Demo => self.insert_viewer_element(ProgramDemo::new(&self.context)?),
                    ViewerElementType::VortonRender => self.insert_viewer_element(ProgramVortonRender::new(&self.context)?),
                };
                Ok(id)
            },
            Err(e) => Err(Box::new(SimpleError::new(format!("Unable to identify element to created from string {}. Error: {}", content, e).as_str()))),
        }
    }

}
