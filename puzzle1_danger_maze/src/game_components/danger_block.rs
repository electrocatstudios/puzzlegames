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

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        
    }
}