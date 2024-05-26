use core::num;

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use rand::Rng;
use gloo_console::log;

use crate::utils::{Circle, Point};
use crate::game_control::GAME_HEIGHT;

use super::confetti::Confetti;

pub struct Goal {
    pub circle: Circle<f64>,
    growing: bool,
    confetti: Vec::<Confetti>,
    confetti_cooldown: f64
}

const GOAL_SIZE: f64 = 40.0;
const GOAL_SPEED: f64 = 0.008;
const GOAL_MIN_SIZE: f64 = 25.0;
const CONFETTI_COOLDOWN: f64 = 250.0;

impl Goal {
    pub fn new(x: f64, y: f64) -> Self {
        Goal {
            circle: Circle::new(x,y,GOAL_SIZE),
            growing: false,
            confetti: Vec::new(),
            confetti_cooldown: CONFETTI_COOLDOWN
        }
    }

    pub fn update(&mut self, delta: f64, win_state: bool) {
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

        if win_state {
            if self.confetti_cooldown > 0.0 {
                self.confetti_cooldown -= delta;
            } else {
                self.confetti_cooldown = CONFETTI_COOLDOWN;

                let mut rng = rand::thread_rng();
                let num_pieces = rng.gen_range(1..5);
                for _ in 0..num_pieces {
                    let x_var = rng.gen_range(-40.0..40.0);
                    let fr = rng.gen_range(0.05..0.15);
                    self.confetti.push(Confetti::new(self.circle.loc.x + x_var, self.circle.loc.y, fr));
                }
                
            }
        } else {
            if self.confetti.len() > 0 {
                self.confetti = Vec::new();
            }
        }

        for con in self.confetti.iter_mut() {
            con.update(delta);
        }
        self.confetti.retain(|ci| {
            ci.ttl > 0.0
        });

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

        let _ = ctx.set_fill_style(&JsValue::from("rgb(0, 255, 255)"));
        
        for con in self.confetti.iter_mut() {
            con.render(ctx);
        }
    }
}