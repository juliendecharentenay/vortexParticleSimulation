use std::{
    collections::HashMap,
    error::Error
};
use web_sys::console;
use wasm_bindgen::prelude::*;
use nalgebra::{Point3, Vector3, Isometry3, Matrix3, Matrix4};
use serde::Deserialize;

mod camera;
use camera::Camera;

#[derive(Deserialize, Clone, Debug)]
pub struct Touch {
    pub x: f32,
    pub y: f32,
    pub id: u32,
}

enum UserEvent {
    MouseDown { x: f32, y: f32 },
    MouseUp   { x: f32, y: f32 },
    MouseMove { x: f32, y: f32 },
    Wheel     { x: f32, y: f32, delta_y: f32 },
}

#[wasm_bindgen]
pub struct CameraBuilder {
    width:    f32,
    height:   f32,
    fov:      f32,
    view:     Matrix4<f32>,
    modifier: Matrix4<f32>,
    orbit:    nalgebra::Rotation3<f32>,
    distance: f32,
    eye:      Point3<f32>,
    target:   Point3<f32>,
    up:       Vector3<f32>,

    mouse_down: Option<UserEvent>,
    touches_down: HashMap<u32, Touch>,
    touches:    HashMap<u32, Touch>,
}

#[wasm_bindgen]
impl CameraBuilder {
    pub fn new(width: f32, height: f32) -> Result<CameraBuilder, JsValue> {
        let eye    = Point3::new(0.0, 5.0, 0.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let up     = Vector3::new(0.0, 0.0, 1.0);
        let fov    = 45.0 * std::f32::consts::PI / 180f32;

        let view: Matrix4<f32> = Isometry3::<f32>::look_at_rh(&eye, &target, &up).to_homogeneous();
        let modifier = Matrix4::<f32>::identity();

        Ok( CameraBuilder { width, 
            height, 
            fov, 
            view, 
            modifier, 
            distance: nalgebra::distance(&eye, &target), 
            eye, 
            target, 
            up, 
            orbit: nalgebra::Rotation3::<f32>::identity(),
            mouse_down: None,
            touches_down: HashMap::new(),
            touches: HashMap::new(),
        } )
    }

    pub fn on_mouse_down(&mut self, x: f32, y: f32) -> Result<(), JsValue> {
        self.handle_user_event(UserEvent::MouseDown { x, y })
            .map_err(|e| JsValue::from_str(format!("Error in on_mouse_down: {}", e).as_str()) )
    }

    pub fn on_mouse_move(&mut self, x: f32, y: f32) -> Result<(), JsValue> {
        self.handle_user_event(UserEvent::MouseMove { x, y })
            .map_err(|e| JsValue::from_str(format!("Error in on_mouse_down: {}", e).as_str()) )
    }

    pub fn on_mouse_up(&mut self, x: f32, y: f32) -> Result<(), JsValue> {
        self.handle_user_event(UserEvent::MouseUp { x, y })
            .map_err(|e| JsValue::from_str(format!("Error in on_mouse_down: {}", e).as_str()) )
    }

    pub fn on_wheel(&mut self, x: f32, y: f32, delta_y: f32) -> Result<(), JsValue> {
        self.handle_user_event(UserEvent::Wheel { x, y, delta_y })
            .map_err(|e| JsValue::from_str(format!("Error in on_mouse_down: {}", e).as_str()) )
    }

    fn touch_mid(touches: &HashMap<u32, Touch>) -> (f32, f32) {
        let alpha: f32 = 1f32 / if (touches.len() > 0) { touches.len() as f32 } else { 1f32 };
        touches.iter().fold((0f32, 0f32), |r, (_, t)| (r.0 + alpha*t.x, r.1 + alpha*t.y))
    }

    fn touch_delta(touches: &HashMap<u32, Touch>, mid: &(f32, f32)) -> f32 {
        touches.iter()
            .fold(0f32, |r, (_, t)| { r + ((t.x - mid.0).powi(2) + (t.y - mid.1).powi(2)).sqrt() })
    }

    fn alpha(a: &Touch, b: &Touch) -> f32 {
        let l = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
        if l > 1e-5 {
            if b.y > a.y {
                1f32 * ((b.x - a.x)/l).acos()
            } else {
                -1f32 * ((b.x - a.x)/l).acos()
            }
        } else {
            0f32
        }
    }

    fn touch_alpha(touches_from: &HashMap<u32, Touch>, touches_to: &HashMap<u32, Touch>) -> f32 {
        let mut reference: Option<(&Touch, &Touch)> = None;
        let mut alpha = 0f32; let mut count = 0;
        for (k, v_from) in touches_from.iter() {
            if let Some(v_to) = touches_to.get(k) {
                if reference.is_none() { 
                    reference = Some((v_from, v_to)); 
                } else {
                    count += 1;
                    alpha += CameraBuilder::alpha(reference.unwrap().1, v_to) - CameraBuilder::alpha(reference.unwrap().0, v_from);
                }
            }
        }
        alpha / if (count > 0) { count as f32 } else { 1f32 }
    }

    fn touch_modify(&mut self) -> Result<(), Box<dyn Error>> {
        let (fr_x, fr_y) = CameraBuilder::touch_mid(&self.touches_down);
        let fr_l         = CameraBuilder::touch_delta(&self.touches_down, &(fr_x, fr_y));

        let (to_x, to_y) = CameraBuilder::touch_mid(&self.touches);
        let to_l         = CameraBuilder::touch_delta(&self.touches, &(to_x, to_y));
        let alpha        = CameraBuilder::touch_alpha(&self.touches_down, &self.touches);

        self.modifier = 
            self.zoom_matrix4(to_x, to_y, to_l - fr_l)
            * Matrix4::<f32>::from_euler_angles(0f32, 0f32, alpha)
            * self.orbit_matrix4(fr_x, fr_y, to_x, to_y);

        Ok(())
    }

    fn touch_apply(&mut self) -> Result<(), Box<dyn Error>> {
        self.view = self.modifier * self.view;
        self.modifier = Matrix4::<f32>::identity();
        self.touches_down = self.touches.clone();
        Ok(())
    }

    pub fn on_touch_start(&mut self, touches: JsValue) -> Result<(), JsValue> {
        if self.touches_down.len() > 0 {
            self.touch_modify().map_err(|e| JsValue::from_str(format!("{}", e).as_str()))?; 
            self.touch_apply().map_err(|e| JsValue::from_str(format!("{}", e).as_str()))?;
        }

        let touches: Vec<Touch> = touches.into_serde().map_err(|e| JsValue::from_str(format!("{:?}",e).as_str()))?;
        for touch in touches.iter() {
            self.touches_down.insert(touch.id, touch.clone());
            self.touches.insert(touch.id, touch.clone());
        }
        console::log_1(&JsValue::from_str(format!("on touch start: {:?}", self.touches_down).as_str()));
        Ok(())
    }

    pub fn on_touch_end(&mut self, touches: JsValue) -> Result<(), JsValue> {
        if self.touches_down.len() > 0 {
            self.touch_modify().map_err(|e| JsValue::from_str(format!("{}", e).as_str()))?; 
            self.touch_apply().map_err(|e| JsValue::from_str(format!("{}", e).as_str()))?;
        }
        
        let touches: Vec<Touch> = touches.into_serde().map_err(|e| JsValue::from_str(format!("{:?}",e).as_str()))?;
        for touch in touches.iter() {
            self.touches_down.remove(&touch.id);
            self.touches.remove(&touch.id);
        }
        console::log_1(&JsValue::from_str(format!("on touch end: {:?}", self.touches).as_str()));
        Ok(())
    }

    pub fn on_touch_cancel(&mut self, touches: JsValue) -> Result<(), JsValue> {
        self.on_touch_end(touches)
    }

    pub fn on_touch_move(&mut self, touches: JsValue) -> Result<(), JsValue> {
        let mut touches: Vec<Touch> = touches.into_serde().map_err(|e| JsValue::from_str(format!("{:?}",e).as_str()))?;
        for touch in touches.into_iter() {
            self.touches.insert(touch.id, touch);
        }
        if self.touches_down.len() > 0 {
            self.touch_modify().map_err(|e| JsValue::from_str(format!("{}", e).as_str()))?; 
        }
        Ok(())
    }

    fn handle_user_event(&mut self, event: UserEvent) -> Result<(), Box<dyn Error>> {
        match event {
            UserEvent::Wheel { x, y, delta_y }
            => { // Zoom in and Out event
                /*
                let theta_x = ( x - 0.5*self.width)  / self.height * self.fov;
                let theta_y = ( y - 0.5*self.height) / self.height * self.fov;
                let translation = Vector3::<f32>::new(0.0, 0.0, delta_y / self.height * 1.0 * self.distance);
                let rotation = nalgebra::Rotation3::<f32>::from_euler_angles(-theta_y, -theta_x, 0.0);
                self.view = Matrix4::<f32>::from(
                        nalgebra::Translation3::<f32>::from( rotation * translation)
                     ) * self.view;
                     */
                self.view = self.zoom_matrix4(x, y, delta_y) * self.view;
                Ok(())
            },

            UserEvent::MouseDown { x, y }
            => {
                self.mouse_down = Some( UserEvent::MouseDown { x, y } );
                Ok(())
            },

            UserEvent::MouseMove { x, y }
            => {
                if self.mouse_down.is_some() {
                    let to_x = x; let to_y = y;
                    if let UserEvent::MouseDown { x, y } = self.mouse_down.as_ref().unwrap() {
                        self.modifier = self.orbit_matrix4(*x, *y, to_x, to_y);
                    }
                }
                Ok(())
            },

            UserEvent::MouseUp { x, y }
            => {
                if self.mouse_down.is_some() {
                    let to_x = x; let to_y = y;
                    if let UserEvent::MouseDown { x, y } = self.mouse_down.as_ref().unwrap() {
                        self.view = self.orbit_matrix4(*x, *y, to_x, to_y) * self.view;
                        self.modifier = Matrix4::<f32>::identity();
                    }
                }
                self.mouse_down = None;
                Ok(())
            },
        }
    }

    fn zoom_matrix4(&self, x: f32, y: f32, delta: f32) -> Matrix4::<f32> {
        let theta_x = ( x - 0.5*self.width)  / self.height * self.fov;
        let theta_y = ( y - 0.5*self.height) / self.height * self.fov;
        let translation = Vector3::<f32>::new(0.0, 0.0, delta / self.height * 1.0 * self.distance);
        let rotation = nalgebra::Rotation3::<f32>::from_euler_angles(-theta_y, -theta_x, 0.0);
        Matrix4::<f32>::from(nalgebra::Translation3::<f32>::from( rotation * translation))
    }

    fn orbit_matrix4(&self, from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> Matrix4::<f32> {
        let theta_x = (from_x - to_x) / self.height * self.fov;
        let theta_y = (from_y - to_y) / self.height * self.fov;
        Matrix4::<f32>::from_euler_angles(theta_y, theta_x, 0.0)
    }

        /*
    fn view(&self) -> Matrix4<f32> {
        Isometry3::<f32>::look_at_rh(&(self.orbit*self.eye), 
                                     &(self.orbit*self.target), 
                                     &(self.orbit*self.up)).to_homogeneous()
        self.view.clone()
    }
                                     */

    pub fn orbit(&mut self, from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> Result<(), JsValue> {
        let theta_x = (from_x - to_x)/self.height * self.fov;
        let theta_y = (from_y - to_y)/self.height * self.fov;
        // self.orbit = nalgebra::Rotation3::<f32>::from_euler_angles(theta_y, 0.0, theta_x);
        self.modifier = Matrix4::<f32>::from_euler_angles(theta_y, theta_x, 0.0);
        Ok(())
    }

    pub fn apply_orbit(&mut self) -> Result<(), JsValue> {
        self.view = self.modifier * self.view;
        self.modifier = Matrix4::<f32>::identity();

        /*
        self.eye    = self.orbit * self.eye;
        self.target = self.orbit * self.target;
        self.up     = self.orbit * self.up;

        self.orbit = nalgebra::Rotation3::<f32>::identity();
        */
        Ok(())
    }

    pub fn apply_translate(&mut self, mid_x: f32, mid_y: f32, x: f32, y: f32, factor: f32) -> Result<(), JsValue> {
        let theta_x = (x - mid_x)/self.height * self.fov;
        let theta_y = (y - mid_y)/self.height * self.fov;
        let translation = Vector3::<f32>::new(0.0, 0.0, factor * self.distance);
        let rotation = nalgebra::Rotation3::<f32>::from_euler_angles(-theta_y, -theta_x, 0.0);
        self.view 
            = Matrix4::<f32>::from(
                nalgebra::Translation3::<f32>::from( rotation * translation)
                )
            * self.view;
        Ok(())
    }
}

impl CameraBuilder {
    pub fn to_matrix4(&self) -> Result<Matrix4<f32>, Box<dyn Error>> {
        let projection = Matrix4::<f32>::new_perspective(
                self.width / self.height , self.fov,
                0.1f32, 200f32);
        Ok( projection * self.modifier * self.view )
    }
}

