use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsValue;
use std::vec::Vec;

use crate::utils::Point;

pub struct MouseHandler {
    pub loc: Point::<f64>,
    pub mouse_down: bool,
    pub mouse_move: Point::<f64>
}

const CLICK_INDICATOR_GROWTH_SPEED: f64 = 0.06;
const CLICK_INDICATOR_MAX_SIZE: f64 = 20.0;

impl MouseHandler {
    pub fn new() -> Self {
        MouseHandler {
            loc: Point::new(0.0,0.0),
            mouse_down: false,
            mouse_move: Point::new(0.0,0.0)
        }
    }

    pub fn update(&mut self, delta: f64) {
        
    }

    pub fn click(&mut self, x: f64, y: f64) {
    }

    pub fn update_pos(&mut self, x: f64, y: f64) {
        self.loc.x = x;
        self.loc.y = y;
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
    
    }
}