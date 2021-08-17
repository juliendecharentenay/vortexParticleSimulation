use std::error::Error;

use web_sys::{WebGl2RenderingContext, WebGlShader, WebGlProgram};
use simple_error::SimpleError;

pub fn webgl_link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, Box<dyn Error>> {
    let program 
        = match context.create_program() {
            Some(p) => p,
            None => return Err(Box::new(SimpleError::new("Unable to create program object"))),
        };

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(Box::new(SimpleError::new(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))))
    }
}

pub fn webgl_compile_vertex_shader(context: &WebGl2RenderingContext,
                                     source: &str) -> Result<WebGlShader, Box<dyn Error>> {
    webgl_compile_shader(context,
                         WebGl2RenderingContext::VERTEX_SHADER,
                         source)
}

pub fn webgl_compile_fragment_shader(context: &WebGl2RenderingContext,
                                     source: &str) -> Result<WebGlShader, Box<dyn Error>> {
    webgl_compile_shader(context,
                         WebGl2RenderingContext::FRAGMENT_SHADER,
                         source)
}

pub fn webgl_compile_shader(context: &WebGl2RenderingContext,
                      shader_type: u32,
                      source: &str,
                      ) -> Result<WebGlShader, Box<dyn Error>> {
    let shader 
        = match context.create_shader(shader_type) {
            Some(s) => s,
            None => return Err(Box::new(SimpleError::new("Unable to create shader object"))),
        };
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(Box::new(SimpleError::new(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))))
    }
}


