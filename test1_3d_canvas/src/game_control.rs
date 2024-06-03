use rand::seq::index;
use web_sys::{ WebGlRenderingContext as GL, WebGlShader, WebGlProgram, HtmlCanvasElement};
use yew::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;
use js_sys::Date;
use gloo_console::log;
use gloo_net::http::Request;

use std::collections::HashMap;

use core::cell::RefCell;
use std::rc::Rc;

use crate::components::square::Square;
use crate::components::mouse_handler::MouseHandler;
use crate::utils;

pub struct GameControl {
    pub mouse: MouseHandler,
    canvas: NodeRef,
    square: Square,
    callback: Closure<dyn FnMut()>,
    last_update: f64,
    cur_time: f64,
    rotation: f64,
    key_list: HashMap<String, bool>,
}

pub enum GameMsg {
    MouseDown((f64, f64)),
    MouseUp((f64,f64)),
    MouseMove((f64,f64)),
    KeyDown(String),
    KeyUp(String),
    Update,
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
            mouse: MouseHandler::new(),
            canvas: NodeRef::default(),
            square: Square::new(0.0,0.0,0.0),
            callback: callback,
            last_update: Date::now(),
            cur_time: 0.0,
            rotation: 0.0,
            key_list: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            GameMsg::MouseDown(evt) => {
                self.mouse.mouse_down = true;
                
                true
            },
            GameMsg::MouseUp(_evt) => {
                self.mouse.mouse_down = false;
                true
            },
            GameMsg::MouseMove(evt) => {
                self.mouse.update_pos(evt.0, evt.1);
                true
            },
            GameMsg::KeyDown(key) => {
                *self.key_list.entry(key).or_insert(true) = true;
                true
            },
            GameMsg::KeyUp(key) => {
                *self.key_list.entry(key).or_insert(false) = false;
                true
            },
            GameMsg::Update => {
                self.game_update();
                false
            }
            GameMsg::Render => {
                self.render_gl(&ctx);
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
                    onkeydown={onkeydown}
                    onkeyup={onkeyup}
                    ref={self.canvas.clone()}
                    tabindex = "1"
                ></canvas>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) { 
        if first_render {
            self.setup_gl();
        }

    }

}

const SQUARE_MOVE_SPEED: f64 = 0.5;

// Code taken from https://github.com/yewstack/yew/blob/master/examples/webgl/src/main.rs 
impl GameControl {

    fn game_update(&mut self) {
        let cur_time = Date::now();
        let diff = cur_time - self.last_update;

        let frac = diff; // / 1000.0; // Fraction of a second
        self.rotation += diff;

        self.cur_time = cur_time;

        if utils::is_key_pressed(&self.key_list, &"KeyA".to_string()) {
            self.square.loc.x -= diff * SQUARE_MOVE_SPEED;
        }
        if utils::is_key_pressed(&self.key_list, &"KeyD".to_string()) {
            self.square.loc.x += diff * SQUARE_MOVE_SPEED;
        }
        if utils::is_key_pressed(&self.key_list, &"KeyW".to_string()) {
            self.square.loc.y += diff * SQUARE_MOVE_SPEED;
        }
        if utils::is_key_pressed(&self.key_list, &"KeyS".to_string()) {
            self.square.loc.y -= diff * SQUARE_MOVE_SPEED;
        }

        if self.square.loc.x > 600.0 {
            self.square.loc.x = 600.0;
        } else if self.square.loc.x < -600.0 {
            self.square.loc.x = -600.0;
        }

        if self.square.loc.y > 350.0 {
            self.square.loc.y = 350.0;
        }
        if self.square.loc.y < -350.0 {
            self.square.loc.y = -350.0;
        }
        self.last_update = cur_time;
        // log!("Square x", self.square.loc.x);
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }
    
    fn setup_gl(&mut self) {
        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();
        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();


        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");

        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(self.square.vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        // self.square.shader = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));
        self.square.shader = Some(shader_program);
        
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer) );
    }

    fn render_gl(&mut self,  ctx: &Context<GameControl>) {

        self.game_update();

        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();
       
        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        let mut timestamp = 0.0;

        // Attach the position vector as an attribute for the GL context.
        match &self.square.shader {
            Some(shader) => {
                log!("Rendering square");
                let position = gl.get_attrib_location(&shader, "coords") as u32 ;
                gl.vertex_attrib_pointer_with_f64(position, 3, GL::FLOAT, false, 0, 0.0);
                gl.enable_vertex_attrib_array(position);
                
                let x = gl.get_uniform_location(&shader, "x");
                gl.uniform1f(x.as_ref(), self.square.loc.x as f32);
                let y = gl.get_uniform_location(&shader, "y");
                gl.uniform1f(y.as_ref(), self.square.loc.y as f32);
                let z = gl.get_uniform_location(&shader, "z");
                gl.uniform1f(z.as_ref(), self.square.loc.z as f32);
                
                // Attach the time as a uniform for the GL context.
                let time = gl.get_uniform_location(&shader, "u_time");
                gl.uniform1f(time.as_ref(), timestamp as f32);
        
                let mut rot = self.rotation;
                let rot_pos = gl.get_uniform_location(&shader, "rot");
                gl.uniform1f(rot_pos.as_ref(), rot as f32);
        
            }, 
            None => {}
        }
        
        gl.viewport(0, 0, GAME_WIDTH as i32, GAME_HEIGHT as i32);
        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        match self.square.shader {
            Some(_) => {
                gl.draw_arrays(GL::TRIANGLES, 0, self.square.vertices.len() as i32 / 3);
                // gl.draw_arrays(GL::TRIANGLES, 0, self.squares.vertices.len());
        
            }, 
            None => {}
        }
           
        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }

}
