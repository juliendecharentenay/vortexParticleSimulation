use std::{
    error::Error,
};

use wasm_bindgen::{
    JsCast,
    JsValue,
};
use wasm_bindgen_futures::{
    JsFuture,
};
use web_sys::{
    Blob,
    ImageBitmap,
    Response,
    WebGlProgram, 
    WebGl2RenderingContext,
    WebGlVertexArrayObject,
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

struct CubeMapTexSxSySw {
    sx: i32,
    sy: i32,
    sw: i32,
}

impl CubeMapTexSxSySw {
    pub fn make_tex(&self,
                    context: &WebGl2RenderingContext,
                    target: u32,
                    source: &ImageBitmap) -> Result<(), Box<dyn Error>> {

        console::log_1(&JsValue::from(format!("TexSxSySw: image {} x {} vs ({} + {} x {} + {})", source.width(), source.height(), self.sx, self.sw, self.sy, self.sw).as_str()));

        let window = web_sys::window()
            .ok_or_else(|| SimpleError::new("Unable to retrieve window"))?;
        let document = window.document()
            .ok_or_else(|| SimpleError::new("Unable to retrieve document"))?;
        let canvas = document.create_element("canvas")
            .map_err(|e| SimpleError::new(format!("{:?}", e).as_str()))?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|e| SimpleError::new(format!("{:?}", e).as_str()))?;
        canvas.set_width(self.sw as u32);
        canvas.set_height(self.sw as u32);
        let ctx = canvas.get_context("2d")
            .map_err(|e| SimpleError::new(format!("{:?}", e).as_str()))?
            .ok_or_else(|| SimpleError::new("Unable to retrieve context"))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|e| SimpleError::new(format!("{:?}", e).as_str()))?;
            
        ctx.draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            source, self.sx as f64, self.sy as f64, self.sw as f64, self.sw as f64, 0.0, 0.0, self.sw as f64, self.sw as f64
            )
            .map_err(|e| SimpleError::new(format!("{:?}", e).as_str()))?;

        context.tex_image_2d_with_u32_and_u32_and_html_canvas_element(
            target,
            0i32, // level
            WebGl2RenderingContext::RGBA as i32, // internal format
            WebGl2RenderingContext::RGBA, // format
            WebGl2RenderingContext::UNSIGNED_BYTE, // type
            &canvas)
            .map_err(|e| SimpleError::new(format!("{:?}", e).as_str()))?;
        Ok(())
    }
}

struct ImageParams {
    neg_x: CubeMapTexSxSySw,
    neg_y: CubeMapTexSxSySw,
    neg_z: CubeMapTexSxSySw,
    pos_x: CubeMapTexSxSySw,
    pos_y: CubeMapTexSxSySw,
    pos_z: CubeMapTexSxSySw,
}

impl ImageParams {
    pub fn assign_textures(&self, context: &WebGl2RenderingContext, source: &ImageBitmap) -> Result<(), Box<dyn Error>> {
        self.neg_x.make_tex(context, WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_X, source)?;
        self.neg_y.make_tex(context, WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Y, source)?;
        self.neg_z.make_tex(context, WebGl2RenderingContext::TEXTURE_CUBE_MAP_NEGATIVE_Z, source)?;
        self.pos_x.make_tex(context, WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_X, source)?;
        self.pos_y.make_tex(context, WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Y, source)?;
        self.pos_z.make_tex(context, WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_Z, source)?;
        Ok(())
    }
}

pub struct ProgramSkyBox {
    program: Option<WebGlProgram>,
    image: ImageBitmap,
    image_params: ImageParams,
    vao: Option<WebGlVertexArrayObject>,
}

impl ProgramSkyBox {
    pub async fn new() -> Result<ProgramSkyBox, Box<dyn Error>> {
        let url = "/vpm/assets/skybox/miramar_large.jpg";
        let image_params = ImageParams {
            neg_x: CubeMapTexSxSySw { sx: 0*1024i32, sy: 1*1024i32, sw: 1024i32 },
            neg_y: CubeMapTexSxSySw { sx: 1*1024i32, sy: 2*1024i32, sw: 1024i32 },
            neg_z: CubeMapTexSxSySw { sx: 3*1024i32, sy: 1*1024i32, sw: 1024i32 },
            pos_x: CubeMapTexSxSySw { sx: 2*1024i32, sy: 1*1024i32, sw: 1024i32 },
            pos_y: CubeMapTexSxSySw { sx: 1*1024i32, sy: 0*1024i32, sw: 1024i32 },
            pos_z: CubeMapTexSxSySw { sx: 1*1024i32, sy: 1*1024i32, sw: 1024i32 },
        };

        let image = ProgramSkyBox::fetch_image(url).await?;
        Ok(ProgramSkyBox {
            program: None,
            image,
            image_params,
            vao: None,
        })
    }

    async fn fetch_image(url: &str) -> Result<ImageBitmap, Box<dyn Error>> {
        let window = web_sys::window()
            .ok_or_else(|| Box::new(SimpleError::new("Unable to get window")))?;

        let resp: Response = JsFuture::from(window.fetch_with_str(url))
            .await
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}", e).as_str())))?
            .dyn_into()
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}", e).as_str())))?;

        let blob: Blob = JsFuture::from(
            resp.blob()
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}", e).as_str())))?
            )
            .await
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}", e).as_str())))?
            .dyn_into()
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}", e).as_str())))?;

        let image_bitmap: ImageBitmap = JsFuture::from(
            window.create_image_bitmap_with_blob(&blob)
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}", e).as_str())))?
            )
            .await
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}", e).as_str())))?
            .dyn_into()
            .map_err(|e| Box::new(SimpleError::new(format!("{:#?}", e).as_str())))?;

        Ok(image_bitmap)
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

        if self.vao.is_none() {
            // Create a vertex array object and make it the one we are working with
            self.vao = Some(context.create_vertex_array()
              .ok_or(Box::new(SimpleError::new("ProgramSkyBox::redraw - Unable to create vertex array object")))?);
            context.bind_vertex_array(self.vao.as_ref());

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

            // Textures
            let texture = context.create_texture();
            context.bind_texture(WebGl2RenderingContext::TEXTURE_CUBE_MAP, texture.as_ref());
            self.image_params.assign_textures(&context, &self.image)?;
            context.generate_mipmap(WebGl2RenderingContext::TEXTURE_CUBE_MAP);
            context.tex_parameteri(WebGl2RenderingContext::TEXTURE_CUBE_MAP,
                                   WebGl2RenderingContext::TEXTURE_MIN_FILTER,
                                   WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32);

        } else {
            context.bind_vertex_array(self.vao.as_ref());
        }

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

