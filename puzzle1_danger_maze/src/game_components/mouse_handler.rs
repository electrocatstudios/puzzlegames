use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsValue;

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

    pub fn update_pos(&mut self, x: f64, y: f64) {
        self.loc.x = x;
        self.loc.y = y;
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        let _ = ctx.set_fill_style(&JsValue::from("rgb(255, 55, 55)"));
        let _ = ctx.begin_path();
        let _ = ctx.arc(
                    self.loc.x,
                    self.loc.y,
                    2.0, 
                    0.0, 
                    std::f64::consts::PI * 2.0
                );
        let _ = ctx.fill();
    }
}