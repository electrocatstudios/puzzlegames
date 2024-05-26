use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;

use crate::game_components::{mouse_handler::MouseHandler, player::Player};

pub struct GameControl {
    pub mouse: MouseHandler,
    pub player: Player,
    canvas: NodeRef,
    callback: Closure<dyn FnMut()>,
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

const GAME_HEIGHT: f64 = 800.0;
const GAME_WIDTH: f64 = 1280.0;

impl Component for GameControl {
    type Message = GameMsg;
    type Properties = GameControlProps;

    fn create(ctx: &Context<Self>) -> Self {
        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(GameMsg::Render)) as Box<dyn FnMut()>);
       
        ctx.link().send_message(GameMsg::Render);
       
        GameControl{
            mouse: MouseHandler::new(),
            player: Player::new(100.0, 100.0),
            canvas: NodeRef::default(),
            callback: callback
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            GameMsg::MouseDown(evt) => {
                self.mouse.mouse_down = true;
                let dist = self.player.dist_from_player(evt.0, evt.1);
                if dist < self.player.player_size() {
                    self.player.is_moving = true;
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
            GameMsg::KeyDown(key) => {
                true
            },
            GameMsg::KeyUp(key) => {
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
        if self.player.is_moving {
            self.player.loc.x = self.mouse.loc.x;
            self.player.loc.y = self.mouse.loc.y;
        }

        self.player.update();
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
    
            self.player.render(&mut ctx);

            self.mouse.render(&mut ctx);
            
            window()
                .unwrap()
                .request_animation_frame(self.callback.as_ref().unchecked_ref())
                .unwrap();
    

    }
}