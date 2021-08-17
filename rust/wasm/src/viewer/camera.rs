use std::error::Error;

use wasm_bindgen::{JsValue};
use web_sys::{console};

use nalgebra::{Isometry3, Point3, Vector3, Matrix4};

pub struct Camera {
    width: u32,
    height: u32,
    eye: Option<Point3<f32>>,
    target: Option<Point3<f32>>,
    up: Option<Vector3<f32>>,
    field_of_view_deg: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        Camera {
            width,
            height,
            eye: None,
            target: None,
            up: None,
            field_of_view_deg: 60f32,
        }
    }

    pub fn set_eye_in_place(&mut self, eye: Point3<f32>) -> Result<(), Box<dyn Error>> {
        self.eye = Some(eye);
        Ok(())
    }

    pub fn set_eye(mut self, eye: Point3<f32>) -> Result<Camera, Box<dyn Error>> {
        self.set_eye_in_place(eye)?;
        Ok(self)
    }

    pub fn set_target_in_place(&mut self, target: Point3<f32>) -> Result<(), Box<dyn Error>> {
        self.target = Some(target);
        Ok(())
    }

    pub fn set_target(mut self, target: Point3<f32>) -> Result<Camera, Box<dyn Error>> {
        self.set_target_in_place(target)?;
        Ok(self)
    }

    pub fn set_up_in_place(&mut self, up: Vector3<f32>) -> Result<(), Box<dyn Error>> {
        self.up = Some(up);
        Ok(())
    }

    pub fn set_up(mut self, up: Vector3<f32>) -> Result<Camera, Box<dyn Error>> {
        self.set_up_in_place(up)?;
        Ok(self)
    }

    pub fn as_view_projection(&self) -> Result<Matrix4<f32>, Box<dyn Error>>
    {
        let view = Isometry3::<f32>::look_at_rh(
            self.eye.as_ref().unwrap(),
            self.target.as_ref().unwrap(),
            self.up.as_ref().unwrap()
            );
        let projection = Matrix4::<f32>::new_perspective(
            self.width as f32 / self.height as f32,
            self.field_of_view_deg * std::f32::consts::PI / 180f32 ,
            0.1f32, 200f32);
        let view_projection = projection * view.to_homogeneous();
        Ok(view_projection)
    }

}
