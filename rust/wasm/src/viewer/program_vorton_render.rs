use std::error::Error;
use web_sys::{WebGlProgram, WebGl2RenderingContext};
use simple_error::SimpleError;

use vortex_particle_simulation::{Simulation};

use super::{
    View,
    Camera,
};
use super::{webgl_link_program, webgl_compile_vertex_shader, webgl_compile_fragment_shader};

pub struct ProgramVortonRender
{
    program: Option<WebGlProgram>,
    vertices: Vec<f32>,
    n_vertices: usize,
}

impl ProgramVortonRender {
    pub fn new() -> Result<ProgramVortonRender, Box<dyn Error>> {
        Ok(ProgramVortonRender {
            program: None,
            vertices: Vec::new(),
            n_vertices: 0,
        })
    }

    fn program(&mut self, context: &WebGl2RenderingContext) -> Result<&WebGlProgram, Box<dyn Error>> {
        if self.program.is_none() {
          let vert_shader = webgl_compile_vertex_shader(
              context,
              r##"
     attribute vec4 vPosition;
     uniform mat4 uMatrix;
     void main()
     {
        gl_Position = uMatrix*vPosition;
        gl_PointSize = 2.5;
     }
            "##,
            )?;


          let frag_shader = webgl_compile_fragment_shader(
              context,
            r##"
     precision mediump float;
     void main()
     {
       gl_FragColor = vec4(0.9, 0.9, 0.9, 1);
     }
            "##,
            )?;
          self.program = Some(webgl_link_program(&context, &vert_shader, &frag_shader)?);
        }
        self.program
            .as_ref()
            .ok_or(Box::new(SimpleError::new("Unable to retrieve program")))
    }
}

impl View for ProgramVortonRender {
    fn reset(&mut self) -> Result<(), Box<dyn Error>> {
        self.program = None;
        Ok(())
    }

    /*
     * Draw the simulation to webgl
     */
    fn draw(&mut self, context: &WebGl2RenderingContext, camera: &Camera, simulation: &Simulation) -> Result<(), Box<dyn Error>> {
        // context.use_program(Some(&self.program));
        let buffer = match context.create_buffer() {
            Some(b) => b,
            None => return Err(Box::new(SimpleError::new("Failed to create buffer"))),
        };
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        self.n_vertices = simulation.vortons().len();
        self.vertices 
            = simulation
            .vortons()
            .iter()
            .fold(
                Vec::new(),
                |mut r, v| {
                    let p = v.position();
                    r.push(p.x as f32);
                    r.push(p.y as f32);
                    r.push(p.z as f32);
                    r
                }
                );

        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(self.vertices.as_slice());

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        
        let vao = match context.create_vertex_array() {
            Some(a) => a,
            None => return Err(Box::new(SimpleError::new("Could not create vertex array object"))),
        };
        context.bind_vertex_array(Some(&vao));

        context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        let v_position_location = context.get_attrib_location(self.program(context)?, "vPosition");
        context.enable_vertex_attrib_array(v_position_location as u32);

        context.bind_vertex_array(Some(&vao));

        self.redraw(context, camera)
    }

    fn redraw(&mut self, context: &WebGl2RenderingContext, camera: &Camera) -> Result<(), Box<dyn Error>> {
        context.use_program(Some(self.program(context)?));

        let u_matrix_location = context.get_uniform_location(self.program(context)?, "uMatrix");
        context.uniform_matrix4fv_with_f32_array(
            u_matrix_location.as_ref(),
            false,
            camera.to_matrix4()?.as_slice());
            
        context.draw_arrays(WebGl2RenderingContext::POINTS, 0, self.n_vertices as i32);
        Ok(())
    }
}
