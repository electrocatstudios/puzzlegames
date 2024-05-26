use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use crate::utils::Circle;

pub struct Goal {
    pub circle: Circle<f64>,
    growing: bool
}

const GOAL_SIZE: f64 = 40.0;
const GOAL_SPEED: f64 = 0.008;
const GOAL_MIN_SIZE: f64 = 25.0;

impl Goal {
    pub fn new(x: f64, y: f64) -> Self {
        Goal {
            circle: Circle::new(x,y,GOAL_SIZE),
            growing: false
        }
    }

    pub fn update(&mut self, delta: f64) {
        if !self.growing {
            self.circle.size -= GOAL_SPEED * delta;
            if self.circle.size < GOAL_MIN_SIZE {
                self.growing = true;
            }
        } else {
            self.circle.size += GOAL_SPEED * delta;
            if self.circle.size > GOAL_SIZE {
                self.growing = false;
            }
        }
    }

    pub fn get_dist(&self) -> f64 {
        GOAL_SIZE + 10.0
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        let _ = ctx.set_fill_style(&JsValue::from("rgb(65, 65, 155)"));
        let _ = ctx.set_stroke_style(&JsValue::from("rgb(65, 65, 155)"));

        let _ = ctx.begin_path();
        let _ = ctx.arc(
                    self.circle.loc.x,
                    self.circle.loc.y,
                    GOAL_SIZE + 10.0, 
                    0.0, 
                    std::f64::consts::PI * 2.0
                );
        let _ = ctx.fill();

        let _ = ctx.set_fill_style(&JsValue::from("rgb(165, 165, 255)"));
        let _ = ctx.set_stroke_style(&JsValue::from("rgb(165, 165, 255)"));

        let _ = ctx.begin_path();
        let _ = ctx.arc(
                    self.circle.loc.x,
                    self.circle.loc.y,
                    self.circle.size, 
                    0.0, 
                    std::f64::consts::PI * 2.0
                );
        let _ = ctx.fill();
    }
}