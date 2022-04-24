use std::{
    error::Error,
    rc::Rc,
};
use futures::executor::block_on;

use wasm_bindgen::{
    closure::Closure,
    JsCast,
    JsValue,
};
use wasm_bindgen_futures::{
    JsFuture,
    spawn_local,
};
use web_sys::{
    Blob,
    HtmlImageElement,
    ImageBitmap,
    Response,
    WebGlProgram, 
    WebGl2RenderingContext,
    WebGlTexture,
    console,
};

use nalgebra::{
    Matrix4,
};
use simple_error::SimpleError;

use vortex_particle_simulation::Simulation;

use super::{
    Camera,
    View,
    webgl_link_program, webgl_compile_vertex_shader,
    webgl_compile_fragment_shader,
};

pub struct ProgramSkyBox {
    program: Option<WebGlProgram>,
    has_texture_cube_map: bool,
}

impl ProgramSkyBox {
    pub fn new() -> Result<ProgramSkyBox, Box<dyn Error>> {
        Ok(ProgramSkyBox {
            program: None,
            has_texture_cube_map: false,
        })
    }

    fn program(&mut self, context: &WebGl2RenderingContext) -> Result<&WebGlProgram, Box<dyn Error>> {
        if self.program.is_none() {
            let vert_shared = webgl_compile_vertex_shader(
                context,
                r##"#version 300 es
                in vec4 a_position;
                out vec4 v_position;
                void main() {
                  v_position = a_position;
                  gl_Position = vec4(a_position.xy, 1, 1);
                }
                "##,
                )?;

             let frag_shader = webgl_compile_fragment_shader(
                 context,
                 r##"#version 300 es
                 precision highp float;
                 uniform samplerCube u_skybox;
                 uniform mat4 u_viewDirectionProjectionInverse;
                 in vec4 v_position;
                 // we need to declare an output for the fragment shader
                 out vec4 outColor;
                 void main() {
                   vec4 t = u_viewDirectionProjectionInverse * v_position;
                   outColor = texture(u_skybox, normalize(t.xyz / t.w));
                 }
                 "##,
                 )?;

             self.program = Some(webgl_link_program(&context, &vert_shared, &frag_shader)?);
        }
        self.program
            .as_ref()
            .ok_or(Box::new(SimpleError::new("ProgramSkyBox::program - Unable to retrieve program")))
    }

}

impl View for ProgramSkyBox {
    fn reset(&mut self) -> Result<(), Box<dyn Error>> {
        self.program = None;
        Ok(())
    }

    fn draw(&mut self, context: &WebGl2RenderingContext, camera: &Camera, _simulation: &Simulation) -> Result<(), Box<dyn Error>> {
        self.redraw(context, camera)
    }

    fn redraw(&mut self, context: &WebGl2RenderingContext, camera: &Camera) -> Result<(), Box<dyn Error>> {
        context.use_program(Some(self.program(context)?));

        // Create a vertex array object and make it the one we are working with
        let vao = context.create_vertex_array()
            .ok_or(Box::new(SimpleError::new("ProgramSkyBox::redraw - Unable to create vertex array object")))?;
        context.bind_vertex_array(Some(&vao));

        // === Assign Vertex Shader in attribute 'a_position'
        // Set the vertex positions...
        let position_buffer = context.create_buffer()
            .ok_or(Box::new(SimpleError::new("ProgramSkyBox::redraw - Unable to create buffer for attribute 'a_position'")))?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));
        // Set geometry
        unsafe {
            let positions = js_sys::Float32Array::view(
                &[
                -1.0, -1.0,
                 1.0, -1.0,
                -1.0,  1.0,
                -1.0,  1.0,
                 1.0, -1.0,
                 1.0,  1.0,
                ]);
            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions,
                WebGl2RenderingContext::STATIC_DRAW,
                );
        }
        // Look-up where the vertex data needs to go
        let position_location = context.get_attrib_location(self.program(context)?, "a_position");
        context.enable_vertex_attrib_array(position_location as u32);
        // Assign 
        context.vertex_attrib_pointer_with_i32(position_location as u32, 
                                               2,      // size
                                               WebGl2RenderingContext::FLOAT, // type
                                               false,  // normalise
                                               0,      // stride
                                               0);     // offset

        // == Assign fragment shader uniform 'v_viewDirectionProjectionInverse'
        let u_view_direction_projection_inverse_location = context.get_uniform_location(self.program(context)?, "u_viewDirectionProjectionInverse");
        // Compute camera
        let mut matrix = camera.to_view_matrix4()?;
        matrix.set_column(3, &nalgebra::Vector4::new(0f32, 0f32, 0f32, 1f32));
        matrix.set_row(3, &nalgebra::RowVector4::new(0f32, 0f32, 0f32, 1f32));
        let matrix = (camera.to_projection_matrix4()? * matrix)
            .try_inverse()
            .ok_or_else(|| Box::new(SimpleError::new("Unable to obtain matrix inverse for viewProjection")))?;
        let matrix = Matrix4::new(1f32, 0f32, 0f32, 0f32,
                                  0f32, 0f32, 1f32, 0f32,
                                  0f32, 1f32, 0f32, 0f32,
                                  0f32, 0f32, 0f32, 1f32,
                                  ) * matrix;
        context.uniform_matrix4fv_with_f32_array(
            u_view_direction_projection_inverse_location.as_ref(),
            false,
            matrix.as_slice());

        // Lookup uniforms
        if ! self.has_texture_cube_map {
            let texture = context.create_texture();
            context.bind_texture(WebGl2RenderingContext::TEXTURE_CUBE_MAP, texture.as_ref());
            ProgramSkyBox::set_empty_texture(&context, 1024, 1024)?;
            { 
                let texture = texture.ok_or_else(|| Box::new(SimpleError::new("Unable to extract texture")))?;
                let texture = Rc::new(texture);
                ProgramSkyBox::load_texture_cube_map_miramar_1(&context, Rc::clone(&texture))?;
            }

            context.generate_mipmap(WebGl2RenderingContext::TEXTURE_CUBE_MAP);
            context.tex_parameteri(WebGl2RenderingContext::TEXTURE_CUBE_MAP,
                                   WebGl2RenderingContext::TEXTURE_MIN_FILTER,
                                   WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32);
            self.has_texture_cube_map = true;
        }
        let u_skybox_location = context.get_uniform_location(self.program(context)?, "u_skybox");
        context.uniform1i(u_skybox_location.as_ref(), 0);

        context.enable(WebGl2RenderingContext::CULL_FACE);
        context.enable(WebGl2RenderingContext::DEPTH_TEST);

        // let our quad pass the depth test at 1.0
        context.depth_func(WebGl2RenderingContext::LEQUAL);

        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
        Ok(())
    }
}

impl ProgramSkyBox {
    fn set_empty_texture(context: &WebGl2RenderingContext, width: i32, height: i32) -> Result<(), Box<dyn Error>> {
        let level = 0;
        let internal_format = WebGl2RenderingContext::RGBA;
        let border = 0;
        let src_format = WebGl2RenderingContext::RGBA;
        let src_type = WebGl2RenderingContext::UNSIGNED_BYTE;

        for target in [
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_X,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_X,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Y,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Z,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Z,
        ].into_iter() {
          // Setup image
          context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            target,
            level,
            internal_format as i32,
            width,
            height,
            border,
            src_format,
            src_type,
            None,
          );
        }
        Ok(())
    }
}
        
impl ProgramSkyBox {
    fn load_texture_cube_map_miramar(
        context: &WebGl2RenderingContext,
        texture: &WebGlTexture
        ) -> Result<(), Box<dyn Error>> {

        let level = 0;
        let internal_format = WebGl2RenderingContext::RGBA;
        let width = 1024;
        let height = 1024;
        let border = 0;
        let src_format = WebGl2RenderingContext::RGBA;
        let src_type = WebGl2RenderingContext::UNSIGNED_BYTE;

        // Setup images
        for target in [
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_X,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_X,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Y,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Z,
            WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Z,
        ].into_iter() {
          context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            target,
            level,
            internal_format as i32,
            width,
            height,
            border,
            src_format,
            src_type,
            None,
          )
              .map_err(|e| Box::new(SimpleError::new(format!("{:?}", e).as_str())))?;
        }

        ProgramSkyBox::request_miramar(&context, &texture);

        Ok(())
    }

    async fn request_miramar(
        context: &WebGl2RenderingContext,
        texture: &WebGlTexture
        ) -> Result<(), Box<dyn Error>> {

        let window = web_sys::window().ok_or_else(|| Box::new(SimpleError::new("Unable to get window")))?;
        let resp_value = JsFuture::from(window.fetch_with_str("/vpm/assets/skybox/miramar_large.jpg"))
            .await
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}",e).as_str())))?;
        let resp: Response = resp_value.dyn_into()
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}",e).as_str())))?;

        let promise = resp.blob()
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}",e).as_str())))?;
        let result = JsFuture::from(promise)
            .await
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}",e).as_str())))?;
        let blob: Blob = result.dyn_into()
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}",e).as_str())))?;

        let promise = window.create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&blob, 0, 0, 1024, 1024)
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}",e).as_str())))?;
        let result = JsFuture::from(promise)
            .await
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}",e).as_str())))?;
        let image_bitmap: ImageBitmap = result.dyn_into()
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}",e).as_str())))?;
        
        Ok(())
    }
}

impl ProgramSkyBox {
    // Inspired from rom https://snoozetime.github.io/2019/12/19/webgl-texture.html
    fn load_texture_cube_map_miramar_1(
        context: &WebGl2RenderingContext,
        texture: Rc<WebGlTexture>,
        ) -> Result<(), Box<dyn Error>> {

        let img_src = "/vpm/assets/skybox/miramar_large.jpg";

        // Load image asynchronously
        let img = HtmlImageElement::new().unwrap();
        let imgrc = Rc::new(img);
        {
            let img = Rc::clone(&imgrc);
            let texture = Rc::clone(&texture);
            let context = Rc::new(context.clone());
            let a = Closure::wrap(Box::new(move || {
                let img = Rc::clone(&img);
                let texture = Rc::clone(&texture);
                let context = Rc::clone(&context);
                spawn_local(async move {
                    let window = web_sys::window().ok_or_else(|| Box::new(SimpleError::new("Unable to get window"))).unwrap();
                    loop { // Use loop hack for early return
                        context.bind_texture(WebGl2RenderingContext::TEXTURE_CUBE_MAP, Some(&texture));
                        let a_sx = 3*1024i32; let a_sy = 1*1024i32;
                        if let Err(e) = ProgramSkyBox::set_image(&context, &window, &img, a_sx, a_sy, WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Z).await {
                            console::log_1(&JsValue::from(format!("{}", e).as_str()));
                            break ;
                        }

                        let a_sx = 1*1024i32; let a_sy = 1*1024i32;
                        if let Err(e) = ProgramSkyBox::set_image(&context, &window, &img, a_sx, a_sy, WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Z).await {
                            console::log_1(&JsValue::from(format!("{}", e).as_str()));
                            break ;
                        }

                        let a_sx = 1*1024i32; let a_sy = 2*1024i32;
                        if let Err(e) = ProgramSkyBox::set_image(&context, &window, &img, a_sx, a_sy, WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Y).await {
                            console::log_1(&JsValue::from(format!("{}", e).as_str()));
                            break ;
                        }

                        let a_sx = 1*1024i32; let a_sy = 0*1024i32;
                        if let Err(e) = ProgramSkyBox::set_image(&context, &window, &img, a_sx, a_sy, WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Y).await {
                            console::log_1(&JsValue::from(format!("{}", e).as_str()));
                            break ;
                        }

                        let a_sx = 0*1024i32; let a_sy = 1*1024i32;
                        if let Err(e) = ProgramSkyBox::set_image(&context, &window, &img, a_sx, a_sy, WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_X).await {
                            console::log_1(&JsValue::from(format!("{}", e).as_str()));
                            break ;
                        }

                        let a_sx = 2*1024i32; let a_sy = 1*1024i32;
                        if let Err(e) = ProgramSkyBox::set_image(&context, &window, &img, a_sx, a_sy, WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_X).await {
                            console::log_1(&JsValue::from(format!("{}", e).as_str()));
                            break ;
                        }


                        context.generate_mipmap(WebGl2RenderingContext::TEXTURE_CUBE_MAP);
                        break ; // Terminate loop hack
                    }
                });
            }) as Box<dyn FnMut()>);
            imgrc.set_onload(Some(a.as_ref().unchecked_ref()));

            // Normally we'd store the handle to later get dropped at an appropriate
            // time but for now we want it to be a global handler so we use the
            // forget method to drop it without invalidating the closure. Note that
            // this is leaking memory in Rust, so this should be done judiciously!
            a.forget();
        }
        imgrc.set_src(img_src);

        Ok(())
    }

    async fn set_image(context: &WebGl2RenderingContext, window: &web_sys::Window, img: &HtmlImageElement, a_sx: i32, a_sy: i32, target: u32) -> Result<(), Box<dyn Error>> {
        let a_sw = 1024i32; let a_sh = 1024i32;
        let i = window.create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(img, a_sx, a_sy, a_sw, a_sh)
            .map_err(|e| Box::new(SimpleError::new(format!("{:?}", e).as_str())))?;

        let i = wasm_bindgen_futures::JsFuture::from(i).await
            .map_err(|e| Box::new(SimpleError::new(format!("{:?}", e).as_str())))?;
        let i: ImageBitmap = i.dyn_into()
            .map_err(|e| Box::new(SimpleError::new(format!("{:?}", e).as_str())))?;

        context.tex_image_2d_with_u32_and_u32_and_image_bitmap(
            target,
            0, // level,
            WebGl2RenderingContext::RGBA as i32, // internal format
            WebGl2RenderingContext::RGBA, // src format
            WebGl2RenderingContext::UNSIGNED_BYTE, // src type
            &i)
            .map_err(|e| Box::new(SimpleError::new(format!("{:?}", e).as_str())))?;
        Ok(())
    }

}

