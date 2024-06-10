use rand::seq::index;
use web_sys::{ WebGlRenderingContext as GL, WebGlShader, WebGlProgram, HtmlCanvasElement};
use yew::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;
use js_sys::Date;
use gloo_console::log;
use gloo_net::http::Request;

use std::collections::HashMap;

use crate::components::cube::Cube;
use crate::components::mouse_handler::MouseHandler;
use crate::utils;

pub struct GameControl {
    pub mouse: MouseHandler,
    canvas: NodeRef,
    cube: Cube,
    background_cubes: Vec::<Cube>,
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
    TouchStart((f64, f64)),
    TouchEnd((f64, f64)),
    TouchMove((f64, f64)),
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

pub const MOUSE_ROTATE_DAMPING_FACTOR: f64 = 0.01;

impl Component for GameControl {
    type Message = GameMsg;
    type Properties = GameControlProps;

    fn create(ctx: &Context<Self>) -> Self {
        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(GameMsg::Render)) as Box<dyn FnMut()>);

        ctx.link().send_message(GameMsg::Render);

        let mut cube_vec = Vec::<Cube>::new();
        cube_vec.push(Cube::new(200.0,200.0, 100.0));

        GameControl{
            mouse: MouseHandler::new(),
            canvas: NodeRef::default(),
            cube: Cube::new(150.0,150.0,0.0),
            background_cubes: cube_vec,
            callback: callback,
            last_update: Date::now(),
            cur_time: 0.0,
            rotation: 0.0,
            key_list: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            GameMsg::MouseDown(_evt) => {
                self.mouse.mouse_down = true;
                true
            },
            GameMsg::MouseUp(_evt) => {
                self.mouse.mouse_down = false;
                true
            },
            GameMsg::MouseMove(evt) => {
                if self.mouse.mouse_down {
                    self.mouse.mouse_move.x = (self.mouse.loc.x - evt.0) * MOUSE_ROTATE_DAMPING_FACTOR;
                    self.mouse.mouse_move.y = (self.mouse.loc.y - evt.1) * MOUSE_ROTATE_DAMPING_FACTOR;
                }
                self.mouse.update_pos(evt.0, evt.1);
                
                true
            },
            GameMsg::TouchStart(evt) => {
                // log!("Event here TouchStart => ", evt.0, evt.1);
                self.mouse.mouse_down = true;
                self.mouse.update_pos(evt.0, evt.1);
                true
            },
            GameMsg::TouchEnd(_evt) => {
                // log!("Event here TouchEnd => ", evt.0, evt.1);
                self.mouse.mouse_down = false;
                true
            },
            GameMsg::TouchMove(evt) => {
                if self.mouse.mouse_down {
                    self.mouse.mouse_move.x = (self.mouse.loc.x - evt.0) * MOUSE_ROTATE_DAMPING_FACTOR;
                    self.mouse.mouse_move.y = (self.mouse.loc.y - evt.1) * MOUSE_ROTATE_DAMPING_FACTOR;
                }
                self.mouse.update_pos(evt.0, evt.1);
                // log!("Event here TouchMove => ", evt.0, evt.1);
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
                // self.game_update();
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
        let ontouchstart = ctx.link().callback(move |evt: TouchEvent | {
            evt.prevent_default();
            match evt.touches().get(0) {
                Some(touch) => GameMsg::TouchStart((touch.page_x() as f64, touch.page_y() as f64)),
                None => GameMsg::Null,
            }
        });
        let ontouchend = ctx.link().callback(move |evt: TouchEvent | {
            evt.prevent_default();
            match evt.touches().get(0) {
                Some(touch) => GameMsg::TouchEnd((touch.page_x() as f64, touch.page_y() as f64)),
                None => GameMsg::Null,
            }
        });
        let ontouchmove = ctx.link().callback(move |evt: TouchEvent | {
            evt.prevent_default();
            match evt.touches().get(0) {
                Some(touch) => GameMsg::TouchMove((touch.page_x() as f64, touch.page_y() as f64)),
                None => GameMsg::Null,
            }
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

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) { 
        if first_render {
            self.setup_gl();
        }
    }

}

const CUBE_MOVE_SPEED: f64 = 0.5;
const CUBE_SPIN_SPEED: f64 = 0.005;
// Code taken from https://github.com/yewstack/yew/blob/master/examples/webgl/src/main.rs 
impl GameControl {

    fn game_update(&mut self) {
        let cur_time = Date::now();
        let diff = cur_time - self.last_update;

        let _frac = diff; // / 1000.0; // Fraction of a second
        self.rotation += diff;

        self.cur_time = cur_time;

        // Movement up/down/left/right
        if utils::is_key_pressed(&self.key_list, &"KeyA".to_string()) {
            self.cube.loc.x -= diff * CUBE_MOVE_SPEED;
        }
        if utils::is_key_pressed(&self.key_list, &"KeyD".to_string()) {
            self.cube.loc.x += diff * CUBE_MOVE_SPEED;
        }
        if utils::is_key_pressed(&self.key_list, &"KeyW".to_string()) {
            self.cube.loc.y -= diff * CUBE_MOVE_SPEED;
        }
        if utils::is_key_pressed(&self.key_list, &"KeyS".to_string()) {
            self.cube.loc.y += diff * CUBE_MOVE_SPEED;
        }

        // Bounds check - for viewport
        if self.cube.loc.x > 1280.0 {
            self.cube.loc.x = 1280.0;
        } else if self.cube.loc.x < 0.0 {
            self.cube.loc.x = 0.0;
        }
        if self.cube.loc.y > 800.0 {
            self.cube.loc.y = 800.0;
        }
        if self.cube.loc.y < 0.0 {
            self.cube.loc.y = 0.0;
        }
        // End Bounds check

        self.cube.rot.x += self.mouse.mouse_move.y;
        self.cube.rot.y += self.mouse.mouse_move.x;

        // Up-down rotation
        if utils::is_key_pressed(&self.key_list, &"KeyT".to_string()) {
            self.cube.rot.x += diff * CUBE_SPIN_SPEED;
        }
        if utils::is_key_pressed(&self.key_list, &"KeyG".to_string()) {
            self.cube.rot.x -= diff * CUBE_SPIN_SPEED;
        }
        // Boundry check up/down rot - limited to +1.0 / -1.0
        if self.cube.rot.x > 1.0 {
            self.cube.rot.x = 1.0;
        }
        if self.cube.rot.x < -1.0 {
            self.cube.rot.x = -1.0;
        }

        // Rotation side-to-side
        if utils::is_key_pressed(&self.key_list, &"KeyY".to_string()) {
            self.cube.rot.y += diff * CUBE_SPIN_SPEED;
        }
        if utils::is_key_pressed(&self.key_list, &"KeyU".to_string()) {
            self.cube.rot.y -= diff * CUBE_SPIN_SPEED;
        }

        // Rotation bounds - always be between 0 and 2*pi
        if self.cube.rot.y > 2.0 * std::f64::consts::PI {
            self.cube.rot.y -= 2.0 * std::f64::consts::PI
        }
        if self.cube.rot.y < 0.0 {
            self.cube.rot.y += 2.0 * std::f64::consts::PI
        }

        self.last_update = cur_time;
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
        let verts = js_sys::Float32Array::from(self.cube.vertices.as_slice());

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
        
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));
        self.cube.shader = Some(shader_program.clone());
        
        for c in self.background_cubes.iter_mut() {
            c.shader = Some(shader_program.clone());
        }

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer) );
    }

    fn render_gl(&mut self,  _ctx: &Context<GameControl>) {

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
        match &self.cube.shader {
            Some(shader) => {
                
                let position = gl.get_attrib_location(&shader, "a_position") as u32 ;
                gl.vertex_attrib_pointer_with_f64(position, 3, GL::FLOAT, false, 0, 0.0);
                gl.enable_vertex_attrib_array(position);
                
                // Calculate the transformation matrix for the cube
                let mut matrix = utils::vec4_projection(GAME_WIDTH as f32, GAME_HEIGHT as f32, 400.0);
                let trans = utils::vec4_translate(self.cube.loc.x as f32, self.cube.loc.y as f32, self.cube.loc.z as f32);
                matrix = utils::matrix4_multiply(matrix, trans);
                matrix =utils::matrix4_multiply(matrix, utils::vec4_x_rotation(self.cube.rot.x as f32));
                matrix = utils::matrix4_multiply(matrix, utils::vec4_y_rotation(self.cube.rot.y as f32));
                matrix = utils::matrix4_multiply(matrix, utils::vec4_z_rotation(self.cube.rot.z as f32));
                
                let matrix_location = gl.get_uniform_location(&shader, "u_matrix");// as u32 ;
                gl.uniform_matrix4fv_with_f32_array(matrix_location.as_ref(), false, &matrix);

            }, 
            None => {}
        }

        for c in self.background_cubes.iter() {
            match &c.shader {
                Some(shader) => {
                    
                    let position = gl.get_attrib_location(&shader, "a_position") as u32 ;
                    gl.vertex_attrib_pointer_with_f64(position, 3, GL::FLOAT, false, 0, 0.0);
                    gl.enable_vertex_attrib_array(position);
                    
                    // Calculate the transformation matrix for the cube
                    let mut matrix = utils::vec4_projection(GAME_WIDTH as f32, GAME_HEIGHT as f32, 400.0);
                    let trans = utils::vec4_translate(c.loc.x as f32, c.loc.y as f32, c.loc.z as f32);
                    matrix = utils::matrix4_multiply(matrix, trans);
                    matrix =utils::matrix4_multiply(matrix, utils::vec4_x_rotation(c.rot.x as f32));
                    matrix = utils::matrix4_multiply(matrix, utils::vec4_y_rotation(c.rot.y as f32));
                    matrix = utils::matrix4_multiply(matrix, utils::vec4_z_rotation(c.rot.z as f32));
                    
                    let matrix_location = gl.get_uniform_location(&shader, "u_matrix");// as u32 ;
                    gl.uniform_matrix4fv_with_f32_array(matrix_location.as_ref(), false, &matrix);
    
                }, 
                None => {}
            } 
        }
        
        gl.viewport(0, 0, GAME_WIDTH as i32, GAME_HEIGHT as i32);
        gl.clear_color(0.1, 0.1, 0.1, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);
        // gl.enable(GL::CULL_FACE);
        gl.enable(GL::DEPTH_TEST);
        match self.cube.shader {
            Some(_) => {
                gl.draw_arrays(GL::TRIANGLES, 0, self.cube.vertices.len() as i32 / 3);
            },
            None => {}
        }
           
        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }

}
