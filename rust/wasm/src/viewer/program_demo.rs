use std::error::Error;
use web_sys::{console, WebGlProgram, WebGl2RenderingContext};
use simple_error::SimpleError;

use vortex_particle_simulation::{Simulation};

use crate::viewer::{ViewerElement, webgl_link_program, webgl_compile_vertex_shader, webgl_compile_fragment_shader};
use crate::viewer::camera::Camera;

pub struct ProgramDemo
{
    // context: &'a WebGl2RenderingContext,
    program: WebGlProgram,
    vertices: [f32; 9],
}

impl ProgramDemo {
    pub fn new(context: & WebGl2RenderingContext) -> Result<ProgramDemo, Box<dyn Error>> {
        let vert_shader = webgl_compile_vertex_shader(
            context,
            r##"#version 300 es

            in vec4 position;
            void main() {
              gl_Position = position;
            }
            "##,
        )?;

        let frag_shader = webgl_compile_fragment_shader(
            context,
            r##"#version 300 es

            precision highp float;
            out vec4 outColor;
            void main() {
              outColor = vec4(0.9, 0.9, 0.9, 1);
            }
            "##,
        )?;

        let program = webgl_link_program(&context, &vert_shader, &frag_shader)?;

        Ok(ProgramDemo {
            program,
            vertices: [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0],
        })
    }

}

impl ViewerElement for ProgramDemo {
    fn draw(&mut self, context: &WebGl2RenderingContext, camera: &Camera, simulation: &Simulation) -> Result<(), Box<dyn Error>> {
        // context.use_program(Some(&self.program));

        let position_attribute_location = context.get_attrib_location(&self.program, "position");
        let buffer = match context.create_buffer() {
            Some(b) => b,
            None => return Err(Box::new(SimpleError::new("Failed to create buffer"))),
        };
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&self.vertices);

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
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        context.bind_vertex_array(Some(&vao));

        self.redraw(context, camera)
    }

    fn redraw(&mut self, context: &WebGl2RenderingContext, camera: &Camera) -> Result<(), Box<dyn Error>> {
        context.use_program(Some(&self.program));
        let vert_count = (self.vertices.len() / 3) as i32;
        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
        Ok(())
    }
}
