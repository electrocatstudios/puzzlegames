use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::utils::Rect;

pub struct DangerBlock {
    pub pos: Rect<f64>,
}

impl DangerBlock {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        DangerBlock {
            pos: Rect::new(x, y, w, h)
        }
    }

    pub fn update(&mut self, _delta: f64) {

    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from("rgb(55, 255, 55)"));
        ctx.fill_rect(self.pos.loc.x, self.pos.loc.y, self.pos.width, self.pos.height);
    }

    pub fn point_inside(&self, x: f64, y: f64) -> bool {
        if x > self.pos.loc.x && x < self.pos.loc.x + self.pos.width
            && y > self.pos.loc.y && y < self.pos.loc.y + self.pos.height {
                true
        } else {
            false
        }
    }
}