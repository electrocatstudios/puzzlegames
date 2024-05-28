use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;
use js_sys::Date;
use gloo_console::log;
use std::vec::Vec;

use crate::game_components::goal::Goal;
use crate::game_components::player::Player;
use crate::levels::level_model::{LevelBlockModel, LevelCircleModel, LevelModel};
use crate::utils::Point;
use crate::{game_components::{danger_block::DangerBlock, danger_circle::DangerCircle, mouse_handler::MouseHandler}, utils};

pub struct LevelBuilder {
    state: String,
    pub mouse: MouseHandler,
    start: Point<f64>,
    end: Point<f64>,
    player: Player,
    goal: Goal,
    blocks: Vec::<DangerBlock>, 
    circles: Vec::<DangerCircle>,
    canvas: NodeRef,
    callback: Closure<dyn FnMut()>,
    last_update: f64,
    cur_time: f64,
}

pub enum LevelBuildMsg {
    MouseDown((f64, f64)),
    MouseUp((f64,f64)),
    MouseMove((f64,f64)),
    TouchStart((f64, f64)),
    TouchEnd((f64, f64)),
    TouchMove((f64, f64)),
    KeyDown(String),
    KeyUp(String),
    Render,
    Null
}

const GAME_HEIGHT: f64 = 800.0;
const GAME_WIDTH: f64 = 1280.0;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct LevelBuilderProps;

impl Component for LevelBuilder {
    type Message = LevelBuildMsg;
    type Properties = LevelBuilderProps;

    fn create(ctx: &Context<Self>) -> Self {
        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(LevelBuildMsg::Render)) as Box<dyn FnMut()>);

        ctx.link().send_message(LevelBuildMsg::Render);

        LevelBuilder{
            state: "BLOCKS".to_string(),
            mouse: MouseHandler::new(),
            start: Point::new(0.0,0.0),
            end: Point::new(0.0,0.0),
            player: Player::new(100.0,100.0),
            goal: Goal::new(1000.0, 700.0),
            blocks: Vec::<DangerBlock>::new(), 
            circles: Vec::<DangerCircle>::new(),
            canvas: NodeRef::default(),
            callback: callback,
            last_update: Date::now(),
            cur_time: 0.0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            LevelBuildMsg::MouseDown(evt) => {
                self.mouse.mouse_down = true;

                self.mouse.click(evt.0, evt.1);
                self.start.x = evt.0;
                self.start.y = evt.1;

                if self.state == "PLAYER" {
                    self.player.loc.x = evt.0;
                    self.player.loc.y = evt.1;
                } else if self.state == "GOAL" {
                    self.goal.circle.loc.x = evt.0;
                    self.goal.circle.loc.y = evt.1;
                }

                true
            },
            LevelBuildMsg::MouseUp(_evt) => {
                self.mouse.mouse_down = false;
                true
            },
            LevelBuildMsg::MouseMove(evt) => {
                self.mouse.update_pos(evt.0, evt.1);

                if self.mouse.mouse_down {
                    self.end.x = evt.0;
                    self.end.y = evt.1;
                }

                // log!("Event here => ", self.mouse.loc.x, self.mouse.loc.y);
                true
            },
            LevelBuildMsg::TouchStart(evt) => {
                // log!("Event here TouchStart => ", evt.0, evt.1);
                self.mouse.mouse_down = true;
                true
            },
            LevelBuildMsg::TouchEnd(_evt) => {
                // log!("Event here TouchEnd => ", evt.0, evt.1);
                self.mouse.mouse_down = false;
              true
            },
            LevelBuildMsg::TouchMove(evt) => {
                self.mouse.update_pos(evt.0, evt.1);
                // log!("Event here TouchMove => ", evt.0, evt.1);
                true
            },
            LevelBuildMsg::KeyDown(key) => {
                if key == "KeyC" {
                    self.state = "CIRCLES".to_string();
                } else if key == "KeyB" {
                    self.state = "BLOCKS".to_string();
                } else if key == "KeyP" {
                    self.state = "PLAYER".to_string();
                } else if key == "KeyG" {
                    self.state = "GOAL".to_string();
                } else if key == "KeyQ" {
                    self.blocks = Vec::new();
                    self.circles = Vec::new();
                } else if key == "KeyS" {
                    self.save_data();
                }
                true
            },
            LevelBuildMsg::KeyUp(_key) => {
                true
            },
            LevelBuildMsg::Render => {
                self.render();
                true
            },
            LevelBuildMsg::Null => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousedown = ctx.link().callback(move |evt: MouseEvent| {
            LevelBuildMsg::MouseDown((evt.page_x() as f64, evt.page_y() as f64))
        });
        let onmousemove = ctx.link().callback(move |evt: MouseEvent| {
            LevelBuildMsg::MouseMove((evt.page_x() as f64, evt.page_y() as f64))
        });
        let onmouseup = ctx.link().callback(move |evt: MouseEvent| {
            LevelBuildMsg::MouseUp((evt.page_x() as f64, evt.page_y() as f64))
        });
        let ontouchstart = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => LevelBuildMsg::TouchStart((touch.page_x() as f64, touch.page_y() as f64)),
                None => LevelBuildMsg::Null,
            }
        });
        let ontouchend = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => LevelBuildMsg::TouchEnd((touch.page_x() as f64, touch.page_y() as f64)),
                None => LevelBuildMsg::Null,
            }
        });
        let ontouchmove = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => LevelBuildMsg::TouchMove((touch.page_x() as f64, touch.page_y() as f64)),
                None => LevelBuildMsg::Null,
            }
        });
        let onkeydown = ctx.link().callback(move |evt: KeyboardEvent| {
            LevelBuildMsg::KeyDown(evt.code()) 
        });
        let onkeyup = ctx.link().callback(move |evt: KeyboardEvent| {
            LevelBuildMsg::KeyUp(evt.code())
        });

        html! { 
            <div class="game_canvas">
                <canvas id="canvas"
                    style={"margin: 0px; width:1280px; height: 800px; left:0px; top:0px;"}
                    onmousedown={onmousedown}
                    onmousemove={onmousemove}
                    onmouseup={onmouseup}
                    ontouchstart={ontouchstart}
                    ontouchend={ontouchend}
                    ontouchmove={ontouchmove}
                    onkeydown={onkeydown}
                    onkeyup={onkeyup}
                    ref={self.canvas.clone()}
                    tabindex = "1"
                ></canvas>
            </div>
        }
    }
    
}

impl LevelBuilder {
    fn game_update(&mut self) {
        let cur_time = Date::now();
        let diff = cur_time - self.last_update;
        
        self.cur_time += diff;

        self.last_update = cur_time;

        self.mouse.update(diff);

        
        if self.mouse.mouse_down == false
             && self.start.x != 0.0 && self.start.y != 0.0 
             && self.end.x != 0.0 && self.end.y != 0.0 {
            // Save the new block
            if self.state == "BLOCKS" {
                self.blocks.push(
                    DangerBlock::new(
                        self.start.x,
                        self.start.y,
                        self.end.x - self.start.x,
                        self.end.y - self.start.y
                    )
                );
            } else if self.state == "CIRCLES" {
                let rad = utils::dist_between_points(
                    Point::new(self.start.x, self.start.y),
                    Point::new(self.end.x, self.end.y)
                );
                self.circles.push(
                    DangerCircle::new(
                        self.start.x,
                        self.start.y,
                        rad
                    )
                );
            }
            

            self.start.x = 0.0;
            self.start.y = 0.0;
            self.end.x = 0.0;
            self.end.y = 0.0;
        }
    
    }

    fn render(&mut self) {
        self.game_update();

        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        
        // Make sure the we reset the draw surface to prevent stretching
        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        let mut ctx: CanvasRenderingContext2d =
            canvas.get_context("2d").unwrap().unwrap().unchecked_into();

        ctx.set_fill_style(&JsValue::from("rgb(55, 55, 55)"));
        ctx.fill_rect(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT);

        // Game border
        ctx.set_stroke_style(&JsValue::from("rgb(255, 255, 0)"));
        ctx.move_to(0.0, 0.0);
        ctx.line_to(GAME_WIDTH, 0.0);
        ctx.line_to(GAME_WIDTH, GAME_HEIGHT);
        ctx.line_to(0.0, GAME_HEIGHT);
        ctx.line_to(0.0, 0.0);
        ctx.stroke();

        self.mouse.render(&mut ctx);

        ctx.set_font("64px arial");
        utils::drop_shadow_string(&mut ctx, self.state.clone(), 20.0, 780.0);
        
        for block in self.blocks.iter_mut() {
            block.render(&mut ctx);
        }
        for circle in self.circles.iter_mut() {
            circle.render(&mut ctx);
        }


        if self.state == "BLOCKS" {
            if self.mouse.mouse_down && self.end.x != 0.0 && self.end.y != 0.0 {
                ctx.set_fill_style(&JsValue::from("rgb(55, 55, 255)"));
                ctx.fill_rect(
                    self.start.x,
                    self.start.y, 
                    self.end.x - self.start.x,
                    self.end.y - self.start.y
                );
                let _ = ctx.fill();
                // log!("Block pos ", self.end.x - self.start.x, self.end.y - self.start.y);
            }
        } else if self.state == "CIRCLES" {
            if self.mouse.mouse_down && self.end.x != 0.0 && self.end.y != 0.0 {
                let rad = utils::dist_between_points(
                    Point::new(self.start.x, self.start.y),
                    Point::new(self.end.x, self.end.y)
                );

                ctx.set_fill_style(&JsValue::from("rgb(55, 55, 255)"));
                let _ = ctx.begin_path();
                let _ = ctx.arc(
                    self.start.x,
                    self.start.y,
                    rad, 
                    0.0, 
                    std::f64::consts::PI * 2.0
                );
                let _ = ctx.fill();
                // log!("Block pos ", self.end.x - self.start.x, self.end.y - self.start.y);
            }
        }

        self.player.render(&mut ctx);
        self.goal.render(&mut ctx);

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }

    fn save_data(&self) {
        let mut ret = LevelModel::new();
        ret.player.x = self.player.loc.x;
        ret.player.y = self.player.loc.y;

        ret.goal.x = self.goal.circle.loc.x;
        ret.goal.y = self.goal.circle.loc.y;

        for b in self.blocks.iter() {
            ret.danger_blocks.push(
                LevelBlockModel::new(
                    b.pos.loc.x,
                    b.pos.loc.y,
                    b.pos.width,
                    b.pos.height
                )
            );
        }

        for c in self.circles.iter() {
            ret.danger_circles.push(
                LevelCircleModel::new(
                    c.pos.loc.x,
                    c.pos.loc.y,
                    c.pos.size
                )
            );
        }
        let ret_str = serde_json::to_string(&ret).unwrap();
        log!(ret_str);
    }
}