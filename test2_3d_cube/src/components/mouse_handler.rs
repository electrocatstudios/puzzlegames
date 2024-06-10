use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsValue;
use std::vec::Vec;

use crate::utils::Point;

pub struct MouseHandler {
    pub loc: Point::<f64>,
    pub mouse_down: bool,
}

impl MouseHandler {
    pub fn new() -> Self {
        MouseHandler {
            loc: Point::new(0.0,0.0),
            mouse_down: false,
        }
    }

    pub fn _update(&mut self, _delta: f64) {
    }

    pub fn _click(&mut self, _x: f64, _y: f64) {
    }

    pub fn update_pos(&mut self, x: f64, y: f64) {
        self.loc.x = x;
        self.loc.y = y;
    }

    pub fn _render(&mut self, _ctx: &mut CanvasRenderingContext2d) {
    
    }
}