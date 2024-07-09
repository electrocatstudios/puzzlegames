use js_sys::wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;
use js_sys::Date;
use gloo_console::log;
use std::collections::HashMap;


use crate::utils::Point;
use crate::rope_manager::RopeManager;
use crate::utils::is_key_pressed;
use super::mouse_handler::MouseHandler;

pub struct GameControl {
    rope_manager: RopeManager,
    pub mouse: MouseHandler,
    canvas: NodeRef,
    callback: Closure<dyn FnMut()>,
    last_update: f64,
    cur_time: f64,
    key_list: HashMap<String, bool>,
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

        GameControl{
            rope_manager: RopeManager::new(),
            mouse: MouseHandler::new(),
            canvas: NodeRef::default(),
            callback: callback,
            last_update: Date::now(),
            cur_time: 0.0,
            key_list: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            GameMsg::MouseDown(evt) => {
                self.mouse.mouse_down = true;
                self.mouse.click(evt.0, evt.1);
                
                true
            },
            GameMsg::MouseUp(_evt) => {
                self.mouse.mouse_down = false;
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
                true
            },
            GameMsg::TouchEnd(_evt) => {
                // log!("Event here TouchEnd => ", evt.0, evt.1);
                self.mouse.mouse_down = false;
                true
            },
            GameMsg::TouchMove(evt) => {
                self.mouse.update_pos(evt.0, evt.1);
                // log!("Event here TouchMove => ", evt.0, evt.1);
                true
            },
            GameMsg::KeyDown(key) => {
                *self.key_list.entry(key).or_insert(true) = true;
                true
            },
            GameMsg::KeyUp(key) => {
                *self.key_list.entry(key).or_insert(true) = false;
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
        
        self.cur_time += diff;
        
        self.last_update = cur_time;

        self.mouse.update(diff);
        if is_key_pressed(&self.key_list, &"KeyR".to_string()) {
            // Reset the ball position to start
            self.rope_manager.set_ball_pos(Point::<f32>::new(0.0, 10.0));
        }
        self.rope_manager.update(diff);

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
        
        self.rope_manager.render(&mut ctx);

        self.mouse.render(&mut ctx);

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }

}