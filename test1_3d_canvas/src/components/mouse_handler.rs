use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsValue;
use std::vec::Vec;

use crate::utils::{Circle, Point};

pub struct MouseHandler {
    pub loc: Point::<f64>,
    pub mouse_down: bool,
    click_indicator: Vec::<Circle<f64>>
}

const CLICK_INDICATOR_GROWTH_SPEED: f64 = 0.06;
const CLICK_INDICATOR_MAX_SIZE: f64 = 20.0;

impl MouseHandler {
    pub fn new() -> Self {
        MouseHandler {
            loc: Point::new(0.0,0.0),
            mouse_down: false,
            click_indicator: Vec::new()
        }
    }

    pub fn update(&mut self, delta: f64) {
        for c in self.click_indicator.iter_mut() {
            c.size += delta*CLICK_INDICATOR_GROWTH_SPEED;
        }
        self.click_indicator.retain(|ci| {
            ci.size < CLICK_INDICATOR_MAX_SIZE
        });
    }

    pub fn click(&mut self, x: f64, y: f64) {
        self.click_indicator.push(Circle::<f64>::new(x, y, 1.0));
    }

    pub fn update_pos(&mut self, x: f64, y: f64) {
        self.loc.x = x;
        self.loc.y = y;
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        let _ = ctx.set_fill_style(&JsValue::from("rgb(255, 55, 55)"));
        let _ = ctx.set_stroke_style(&JsValue::from("rgb(255, 55, 55)"));
        
        let _ = ctx.begin_path();
        let _ = ctx.arc(
                    self.loc.x,
                    self.loc.y,
                    2.0, 
                    0.0, 
                    std::f64::consts::PI * 2.0
                );
        let _ = ctx.fill();
        
        for c in self.click_indicator.iter_mut() {
            let _ = ctx.begin_path();
            let _ = ctx.arc(
                    c.loc.x,
                    c.loc.y,
                    c.size, 
                    0.0, 
                    std::f64::consts::PI * 2.0
                );
            let _ = ctx.stroke();
        }

    }
}