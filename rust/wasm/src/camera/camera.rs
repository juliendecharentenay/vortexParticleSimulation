use std::{
    error::Error,
    ops::{Mul, MulAssign},
    cell::Cell,
    rc::Rc,
};

use wasm_bindgen::{
    JsValue, JsCast,
    prelude::Closure,
};
use web_sys::{console};

use nalgebra::{Isometry3, Point3, Vector3, Matrix4};
use simple_error::SimpleError;

use vortex_particle_simulation::{Simulation};

/*
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
*/

pub struct Camera {
    matrix: Matrix4<f32>,
}

impl Camera {
    pub fn new(matrix: Matrix4<f32>) -> Result<Camera, Box<dyn Error>> {
        Ok(Camera { matrix })
    }

    pub fn multiply(&self, b: &Matrix4<f32>) -> Result<Camera, Box<dyn Error>> {
        Camera::new( b.mul(&self.matrix) )
    }

    pub fn as_projection(&self) -> Result<&Matrix4<f32>, Box<dyn Error>> {
        Ok(&self.matrix)
    }

    pub fn into_projection(self) -> Result<Matrix4<f32>, Box<dyn Error>> {
        Ok(self.matrix)
    }
}

#[derive(Clone, std::marker::Copy)]
enum UserAction {
    None,
    MouseDown(i32, i32),
}

pub struct CameraBuilder {
    width:  f32,
    height: f32,
    fov:    f32,

    canvas: web_sys::HtmlCanvasElement,
    camera: Option<Camera>,
    camera_modifier: Rc<Cell<Matrix4<f32>>>,
}

impl CameraBuilder {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<CameraBuilder, Box<dyn Error>> {
        let width  = canvas.width() as f32;
        let height = canvas.height() as f32;
        let fov    = 60.0 * std::f32::consts::PI / 180f32;

        let mut b = CameraBuilder { width, height, fov, canvas, camera: None, camera_modifier: Rc::new(Cell::new(Matrix4::<f32>::identity())) };
        // b.register_events()?;
        Ok(b)
    }

    /*
    fn register_events(&mut self) -> Result<(), Box<dyn Error>> {
        let user_action = Rc::new(Cell::new(UserAction::None));
        {
            let user_action = user_action.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                console::log_1(&JsValue::from_str(format!("onmousedown event - {:?}", event).as_str()));
                user_action.set(UserAction::MouseDown(event.client_x(), event.client_y()));
            }) as Box<dyn FnMut(_)>);
            match self.canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()) {
                Ok(_) => (),
                Err(e) => return Err(Box::new(SimpleError::new(format!("Error when adding mousedown event listener. Err: {:?}", e.as_string()).as_str()))),
            };
            closure.forget();
        }

        {
            let user_action = user_action.clone();
            let camera_modifier = self.camera_modifier.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if let UserAction::MouseDown(x,y) = user_action.get() {
                    console::log_1(&JsValue::from_str(format!("onmousemove from {}, {} to {}, {}", x, y, event.client_x(), event.client_y()).as_str()));
                    // user_action.set(UserAction(event.client_x(), event.client_y()));
                    // let org   = camera_modifier.get().clone();
                    let theta = (event.client_x() - x) as f32 / self.width * self.fov;
                    let b     = Matrix4::<f32>::new_rotation(Vector3::<f32>::new(0.0, 0.0, theta));
                    camera_modifier.get().mul_assign(&b);
                }
            }) as Box<dyn FnMut(_)>);
            match self.canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()) {
                Ok(_) => (),
                Err(e) => return Err(Box::new(SimpleError::new(format!("Error when adding mousedown event listener. Err: {:?}", e.as_string()).as_str()))),
            };
            closure.forget();
        }

        {
            let user_action = user_action.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                if let UserAction::MouseDown(x,y) = user_action.get() {
                    console::log_1(&JsValue::from_str(format!("onmouseup from {}, {} to {}, {}", x, y, event.client_x(), event.client_y()).as_str()));
                    user_action.set(UserAction::None);
                }
            }) as Box<dyn FnMut(_)>);
            self.canvas
                .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
                .map_err(|e| Box::new(SimpleError::new(format!("Error when adding mouseup event listener. Err: {:?}", e.as_string()).as_str())))?;
            closure.forget();
        }

        Ok(())
    }
    */

    pub fn make(&mut self, simulation: &Simulation) -> Result<Camera, Box<dyn Error>> {
        if self.camera.is_none() {
            let eye    = Point3::new(0.0, 5.0, 0.0);
            let target = Point3::new(0.0, 0.0, 0.0);
            let up     = Vector3::new(0.0, 0.0, 1.0);
            let w      = self.canvas.width();
            let h      = self.canvas.height();
            let fov    = self.fov; // 60.0 * std::f32::consts::PI / 180f32;

            let view = Isometry3::<f32>::look_at_rh(&eye, &target, &up);
            let projection = Matrix4::<f32>::new_perspective(
                w as f32 / h as f32, fov,
                0.1f32, 200f32);
            self.camera = Some(Camera::new(projection * view.to_homogeneous())?);
            self.camera_modifier = Rc::new(Cell::new(Matrix4::<f32>::identity()));
        }
        Ok(self.camera
           .as_ref()
           .ok_or(Box::new(SimpleError::new("Unable to retrieve camera")))?
           .multiply(&self.camera_modifier.get())?
           )
    }
}

