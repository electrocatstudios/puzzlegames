use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;
use js_sys::Date;
use gloo_console::log;
use gloo_net::http::Request;

use crate::game_components::image::Image;
use crate::game_components::{danger_block::DangerBlock, goal::Goal, mouse_handler::MouseHandler, player::Player};
use crate::levels::level_model::*;

pub struct GameControl {
    state: String,
    pub mouse: MouseHandler,
    pub player: Player,
    cur_level: i32,
    blocks: Vec::<DangerBlock>,
    images: Vec::<Image>,
    goal: Goal,
    canvas: NodeRef,
    callback: Closure<dyn FnMut()>,
    last_update: f64,
    is_loading: bool,
}

pub enum GameMsg {
    MouseDown((f64, f64)),
    MouseUp((f64,f64)),
    MouseMove((f64,f64)),
    TouchStart((f64, f64)),
    TouchEnd((f64, f64)),
    TouchMove((f64, f64)),
    KeyDown(String),
    KeyUp(String),
    LoadLevel(i32),
    LevelLoad(LevelModel),
    Render,
    Null
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct GameControlProps;

pub const GAME_HEIGHT: f64 = 800.0;
pub const GAME_WIDTH: f64 = 1280.0;

impl Component for GameControl {
    type Message = GameMsg;
    type Properties = GameControlProps;

    fn create(ctx: &Context<Self>) -> Self {
        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(GameMsg::Render)) as Box<dyn FnMut()>);

        ctx.link().send_message(GameMsg::Render);

        let comp_ctx = ctx.link().clone();
        comp_ctx.send_message(GameMsg::LoadLevel(1));

        GameControl{
            state: "PLAY".to_string(),
            mouse: MouseHandler::new(),
            player: Player::new(100.0, 100.0),
            cur_level: 1,
            blocks: Vec::<DangerBlock>::new(),
            images: Vec::<Image>::new(),
            goal: Goal::new(1100.0, 400.0),
            canvas: NodeRef::default(),
            callback: callback,
            last_update: Date::now(),
            is_loading: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            GameMsg::MouseDown(evt) => {
                self.mouse.mouse_down = true;
                let dist = self.player.dist_from_player(evt.0, evt.1);
                if dist < self.player.player_size() {
                    self.player.set_moving();
                    // self.player.is_moving = true;
                } else {
                    self.mouse.click(evt.0, evt.1);
                }
                
                if self.state == "WIN" {
                    self.state = "PLAY".to_string();
                    let comp_ctx = ctx.link().clone();
                    comp_ctx.send_message(GameMsg::LoadLevel(self.cur_level + 1));
                }

                true
            },
            GameMsg::MouseUp(_evt) => {
                self.mouse.mouse_down = false;
                self.player.is_moving = false;
                true
            },
            GameMsg::MouseMove(evt) => {
                self.mouse.update_pos(evt.0, evt.1);
                // log!("Event here => ", self.mousehandler.offset_x, self.mousehandler.offset_y);
                true
            },
            GameMsg::TouchStart(evt) => {
                // log!("Event here TouchStart => ", evt.0, evt.1);
                self.mouse.mouse_down = true;
                let dist = self.player.dist_from_player(evt.0, evt.1);
                if dist < self.player.player_size() {
                    self.player.is_moving = true;
                }
                true
            },
            GameMsg::TouchEnd(_evt) => {
                // log!("Event here TouchEnd => ", evt.0, evt.1);
                self.mouse.mouse_down = false;
                self.player.is_moving = false;
                true
            },
            GameMsg::TouchMove(evt) => {
                self.mouse.update_pos(evt.0, evt.1);
                // log!("Event here TouchMove => ", evt.0, evt.1);
                true
            },
            GameMsg::KeyDown(_key) => {
                true
            },
            GameMsg::KeyUp(_key) => {
                true
            },
            GameMsg::LoadLevel(level_num) => {
                self.is_loading = true;
                let comp_ctx = ctx.link().clone();
                self.cur_level = level_num;
                wasm_bindgen_futures::spawn_local(async move {
                    let lvl_str = format!("assets/levels/level{}.json", level_num);
                    log!(lvl_str.clone());
                    let fetched_level = Request::get(lvl_str.as_str())
                                .send()
                                .await
                                .unwrap()
                                .json::<LevelModel>()
                                .await
                                .unwrap();
                    
                    comp_ctx.send_message(GameMsg::LevelLoad(fetched_level));
                });
                false
            },
            GameMsg::LevelLoad(level_model) => {
                self.player.is_moving = false;
                self.player.loc.x = level_model.player.x;
                self.player.loc.y = level_model.player.y;
                
                self.goal.circle.loc.x = level_model.goal.x;
                self.goal.circle.loc.y = level_model.goal.y;
                
                let mut blocks = Vec::<DangerBlock>::new();
                for b in level_model.danger_blocks.iter() {
                    blocks.push(DangerBlock::new(b.x, b.y, b.w, b.h));
                }
                self.blocks = blocks;
                let mut images = Vec::<Image>::new();
                for i in level_model.images.iter() {
                    images.push(Image::new(i.filename.clone(), i.x, i.y))
                }
                self.images = images;

                let comp_ctx = ctx.link().clone();
                comp_ctx.send_message(GameMsg::Render);
                self.is_loading = false;
                true
            },
            GameMsg::Render => {
                self.render();
                true
            },
            GameMsg::Null => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousedown = ctx.link().callback(move |evt: MouseEvent| {
            GameMsg::MouseDown((evt.page_x() as f64, evt.page_y() as f64))
        });
        let onmousemove = ctx.link().callback(move |evt: MouseEvent| {
            GameMsg::MouseMove((evt.page_x() as f64, evt.page_y() as f64))
        });
        let onmouseup = ctx.link().callback(move |evt: MouseEvent| {
            GameMsg::MouseUp((evt.page_x() as f64, evt.page_y() as f64))
        });
        let ontouchstart = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => GameMsg::TouchStart((touch.page_x() as f64, touch.page_y() as f64)),
                None => GameMsg::Null,
            }
        });
        let ontouchend = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => GameMsg::TouchEnd((touch.page_x() as f64, touch.page_y() as f64)),
                None => GameMsg::Null,
            }
        });
        let ontouchmove = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => GameMsg::TouchMove((touch.page_x() as f64, touch.page_y() as f64)),
                None => GameMsg::Null,
            }
        });
        let onkeydown = ctx.link().callback(move |evt: KeyboardEvent| {
            GameMsg::KeyDown(evt.code()) 
        });
        let onkeyup = ctx.link().callback(move |evt: KeyboardEvent| {
            GameMsg::KeyUp(evt.code())
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

impl GameControl {
    fn game_update(&mut self) {
        let cur_time = Date::now();
        let diff = cur_time - self.last_update;

        self.last_update = cur_time;

        if self.player.is_moving {
            self.player.loc.x = self.mouse.loc.x;
            self.player.loc.y = self.mouse.loc.y;
        }

        self.goal.update(diff, self.state == "WIN");
        self.player.update(diff);
        self.mouse.update(diff);

        for block in self.blocks.iter_mut() {
            block.update(diff);
            
            let points = self.player.get_sample_points();
            for pt in points.iter() {
                if block.point_inside(pt.x, pt.y) {
                    self.player.reset();
                }
            }
        }

        let win_dist = self.goal.get_dist() + self.player.player_size();
        if self.player.dist_from_player(self.goal.circle.loc.x, self.goal.circle.loc.y) < win_dist {
            self.state = "WIN".to_string();
        } 

    }

    fn render(&mut self) {
        if self.is_loading {
            return;
        }

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
        
        // Start game render
        for block in self.blocks.iter_mut() {
            block.render(&mut ctx);
        }

        for image in self.images.iter_mut() {
            image.render(&mut ctx);
        }

        self.goal.render(&mut ctx);

        if self.state == "PLAY" {
            self.player.render(&mut ctx);
        } else if self.state == "WIN" {
            ctx.set_fill_style(&JsValue::from("rgb(0,0,0)"));
            ctx.set_font("128px arial");
            let load_string = "GOAL";
            let _ = ctx.fill_text(load_string, 305.0, 355.0);

            ctx.set_fill_style(&JsValue::from("rgb(255,0,0)"));
            let _ = ctx.fill_text(load_string, 300.0, 350.0);

            ctx.set_font("64px arial");
            ctx.set_fill_style(&JsValue::from("rgb(0,0,0)"));
            let load_string = "Click to continue";
            let _ = ctx.fill_text(load_string, 303.0, 453.0);
            ctx.set_fill_style(&JsValue::from("rgb(255,0,0)"));
            let _ = ctx.fill_text(load_string, 300.0, 450.0);
        }
        self.mouse.render(&mut ctx);

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}