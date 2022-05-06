use std::{
    collections::HashMap,
    error::Error,
};

use wasm_bindgen::{JsValue, JsCast};
use web_sys::{
    console,
    HtmlCanvasElement, 
    WebGl2RenderingContext, WebGlProgram, WebGlShader,
};
use simple_error::SimpleError;
use serde::{Deserialize};
use uuid::Uuid;
use nalgebra::Matrix4;

use vortex_particle_simulation::{Simulation};

use crate::{Solution, Camera};

mod program_vorton_render;
use program_vorton_render::ProgramVortonRender;
mod program_skybox;
use program_skybox::ProgramSkyBox;

mod webgl;
use webgl::{webgl_link_program, webgl_compile_vertex_shader, webgl_compile_fragment_shader};

pub trait View {
    fn reset(&mut self) -> Result<(), Box<dyn Error>>;
    fn draw(&mut self, context: &WebGl2RenderingContext, camera: &Camera, simulation: &Simulation) -> Result<(), Box<dyn Error>>;
    fn redraw(&mut self, context: &WebGl2RenderingContext, camera: &Camera) -> Result<(), Box<dyn Error>>;
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ViewType {
    VortonRender,
    SkyBox,
}

pub struct Viewer {
    canvas: HtmlCanvasElement,
    views: HashMap<Uuid, Box<dyn View>>,
}

impl Viewer {
    pub async fn create_view(&mut self, data: &str) -> Result<Uuid, Box<dyn Error>> {
        let uuid = Uuid::new_v4();
        let view = Viewer::to_view(data).await?;
        self.views.insert(uuid.clone(), view);
        Ok(uuid)
    }

    /*
    pub fn read_view(&self, uuid: &Uuid) -> Result<Box<dyn View>, Box<dyn Error>> {
    }

    pub fn update_view(&mut self, uuid: &Uuid, data: &str) -> Result<(), Box<dyn Error>> {
    }

    pub fn delete_view(&mut self, uuid: &Uuid) -> Result<(), Box<dyn Error>> {
    }
    */

    async fn to_view(data: &str) -> Result<Box<dyn View>, Box<dyn Error>> {
        let view: Box<dyn View> = match serde_json::from_str(data)? {
            ViewType::VortonRender => Box::new(ProgramVortonRender::new()?),
            ViewType::SkyBox => Box::new(ProgramSkyBox::new().await?),
        };
        Ok(view)
    }
}

impl Viewer {
    pub fn width(&self) -> u32 { self.canvas.width() }
    pub fn height(&self) -> u32 { self.canvas.height() }
}

impl Viewer {
    pub fn from_element_id(element_id: &str) -> Result<Viewer, Box<dyn Error>> {
        let document = web_sys::window()
            .ok_or_else(|| Box::new(SimpleError::new("Unable to retrieve window")))?
            .document()
            .ok_or_else(|| Box::new(SimpleError::new("Unable to retrieve document")))?;
        let canvas = document.get_element_by_id(element_id)
            .ok_or_else(|| Box::new(SimpleError::new(format!("Unable to retrieve element {}", element_id).as_str())))?;
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|e| Box::new(SimpleError::new(format!("Unable to cast element {} into HtmlCanvasElement. Error: {:?}", element_id, e).as_str())))?;
        Ok(Viewer { 
               canvas,
               views: HashMap::new(),
        })
    }

    pub fn draw(&mut self, solution: &Solution, camera: &Camera) -> Result<(), Box<dyn Error>> {
        let mut context: WebGl2RenderingContext
            = self.canvas
            .get_context("webgl2")
            .map_err(|e| Box::new(SimpleError::new(format!("Viewer::draw - Error retrieving webgl2 context. Error: {:?}", e).as_str())))?
            .ok_or_else(|| Box::new(SimpleError::new("Unable to retrieve webgl2 context")))?
            .dyn_into()
            .map_err(|e| Box::new(SimpleError::new(format!("Viewer::draw - Unable to cast into WebGl2RenderingContext. Error: {:?}", e))))?;

        context.viewport(0, 0, self.width().try_into()?, self.height().try_into()?);
        context.clear_color(6f32/255f32, 78f32/255f32, 59f32/255f32, 1f32);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        for e in self.views.values_mut() { e.draw(&context, camera, solution.simulation_ref())?; }
        Ok(())
    }
}

