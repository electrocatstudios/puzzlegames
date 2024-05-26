use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::utils::Point;

pub struct Player {
    pub loc: Point<f64>,
    pub is_moving: bool
}

const PLAYER_SIZE: f64 = 20.0;

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            loc: Point::<f64>::new(x,y),
            is_moving: false,
        }
    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        if !self.is_moving {
            let _ = ctx.set_fill_style(&JsValue::from("rgb(195, 195, 0)"));
        } else {
            let _ = ctx.set_fill_style(&JsValue::from("rgb(255, 255, 0)"));
        }

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

    pub fn dist_from_player(&self, x: f64, y: f64) -> f64 {
        let diff_x = self.loc.x - x;
        let diff_y = self.loc.y - y;
        let dist = (diff_x*diff_x) + (diff_y*diff_y);
        dist.sqrt()
    }

    pub fn player_size(&self) -> f64 {
        PLAYER_SIZE
    }
}