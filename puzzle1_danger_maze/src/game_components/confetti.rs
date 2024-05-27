use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use rand::Rng;

use crate::utils::Point;

pub struct Confetti {
    loc: Point<f64>,
    color: String,
    pub ttl: f64,
    fall_rate: f64
}

const CONFETTI_TTL: f64 = 3000.0;

impl Confetti {
    pub fn new(x: f64, y: f64, fall_rate: f64) -> Self {
        let mut rng = rand::thread_rng();
        let blu  = rng.gen_range(155..255);
        let grn = rng.gen_range(0..255);
        let red = rng.gen_range(0..85);

        let col_str = "rgb(".to_string() + &red.to_string() + "," + &grn.to_string() + "," + &blu.to_string() + ")";
        Confetti {
            loc: Point::new(x,y),
            color: col_str,
            ttl: CONFETTI_TTL,
            fall_rate: fall_rate
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.loc.y += self.fall_rate * delta;
        self.ttl -= delta;
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        let _ = ctx.set_fill_style(&JsValue::from(self.color.as_str()));
        let _ = ctx.fill_rect(self.loc.x - 5.0, self.loc.y - 5.0, 10.0, 10.0);
    }
}