use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::utils::Circle;

pub struct DangerCircle {
    pub pos: Circle<f64>,
}

impl DangerCircle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        DangerCircle {
            pos: Circle::new(x, y, r)
        }
    }

    pub fn update(&mut self, _delta: f64) {

    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from("rgb(55, 255, 55)"));
        let _ = ctx.begin_path();
        let _ = ctx.arc(
                    self.pos.loc.x,
                    self.pos.loc.y,
                    self.pos.size, 
                    0.0, 
                    std::f64::consts::PI * 2.0
                );
        let _ = ctx.fill();
    }

    pub fn point_inside(&self, x: f64, y: f64, radius: f64) -> bool {
        let diff_x = self.pos.loc.x - x;
        let diff_y = self.pos.loc.y - y;
        let dist = ((diff_x*diff_x) + (diff_y*diff_y)).sqrt();
        if dist < self.pos.size + radius {
            true
        } else {
            false
        }
    }
}