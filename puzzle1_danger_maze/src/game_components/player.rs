use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use std::vec::Vec;

use crate::utils::{Circle, Point};

pub struct Player {
    pub start: Point<f64>,
    pub loc: Point<f64>,
    pub is_moving: bool,
    indicator: Vec::<Circle<f64>>
}

const PLAYER_SIZE: f64 = 20.0;
const PLAYER_INDICATOR_GROWTH_SPEED: f64 = 0.06;
const PLAYER_INDICATOR_MAX_SIZE: f64 = 30.0;

const PLAYER_SAMPLE_NUM: f64 = 8.0;

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Player {
            start: Point::<f64>::new(x,y),
            loc: Point::<f64>::new(x,y),
            is_moving: false,
            indicator: Vec::new()
        }
    }

    pub fn update(&mut self, delta: f64) {
        for c in self.indicator.iter_mut() {
            c.size += delta*PLAYER_INDICATOR_GROWTH_SPEED;
        }
        self.indicator.retain(|ci| {
            ci.size < PLAYER_INDICATOR_MAX_SIZE
        });
    }

    pub fn render(&mut self, ctx: &mut CanvasRenderingContext2d) {
        if !self.is_moving {
            let _ = ctx.set_fill_style(&JsValue::from("rgb(195, 195, 0)"));
            let _ = ctx.set_stroke_style(&JsValue::from("rgb(195, 195, 0)"));
        } else {
            let _ = ctx.set_fill_style(&JsValue::from("rgb(255, 255, 0)"));
            let _ = ctx.set_stroke_style(&JsValue::from("rgb(255, 255, 0)"));
        }

        for c in self.indicator.iter_mut() {
            let _ = ctx.begin_path();
            let _ = ctx.arc(
                    self.loc.x,
                    self.loc.y,
                    c.size, 
                    0.0, 
                    std::f64::consts::PI * 2.0
                );
            let _ = ctx.stroke();
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

    pub fn set_moving(&mut self) {
        self.indicator.push(Circle::<f64>::new(self.loc.x, self.loc.y, PLAYER_SIZE));
        self.is_moving = true;
    }

    pub fn get_sample_points(&self) -> Vec::<Point<f64>> {
        let mut ret = Vec::new();
        let step_size = std::f64::consts::PI * 2.0 / PLAYER_SAMPLE_NUM;
        let mut cur = 0.0;
        while cur < std::f64::consts::PI * 2.0 {
            ret.push(
                Point::new(
                    self.loc.x + (cur.sin() * self.player_size()),
                    self.loc.y + (cur.cos() * self.player_size())
                )
            );
            cur += step_size;
        }
        ret
    }

    pub fn reset(&mut self) {
        self.loc.x = self.start.x;
        self.loc.y = self.start.y;
        self.is_moving = false;
    }

    pub fn set_start(&mut self, x: f64, y: f64) {
        self.start = Point::new(x,y);
        self.loc = Point::new(x,y);
    }
}