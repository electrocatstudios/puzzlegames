use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::utils::Point;

pub struct Player {
    loc: Point<f64>
}
const PLAYER_SIZE: f64 = 20.0;

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            loc: Point::<f64>::new(x,y)
        }
    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        let _ = ctx.set_fill_style(&JsValue::from("rgb(195, 195, 0)"));
        let _ = ctx.begin_path();
        let _ = ctx.arc(
                    self.loc.x,
                    self.loc.y,
                    PLAYER_SIZE, 
                    0.0, 
                    std::f64::consts::PI * 2.0
                );
        let _ = ctx.fill();
        // let _ + ctx.

    }
}