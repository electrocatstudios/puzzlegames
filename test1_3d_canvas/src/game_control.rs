use rand::seq::index;
use web_sys::{ WebGlRenderingContext as GL, WebGlShader, WebGlProgram, HtmlCanvasElement};
use yew::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;
use js_sys::Date;
use gloo_console::log;
use gloo_net::http::Request;

use core::cell::RefCell;
use std::rc::Rc;

use crate::components::mouse_handler::MouseHandler;
use crate::utils;

pub struct GameControl {
    pub mouse: MouseHandler,
    canvas: NodeRef,
    // callback: Closure<dyn FnMut()>,
    last_update: f64,
    cur_time: f64,
    rotation: f64,
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
        // let comp_ctx = ctx.link().clone();
        // let callback =
        //     Closure::wrap(Box::new(move || comp_ctx.send_message(GameMsg::Render)) as Box<dyn FnMut()>);

        // ctx.link().send_message(GameMsg::Render);


        GameControl{
            mouse: MouseHandler::new(),
            canvas: NodeRef::default(),
            // callback: callback,
            last_update: Date::now(),
            cur_time: 0.0,
            rotation: 0.0,
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
                // log!("Event here => ", self.mousehandler.offset_x, self.mousehandler.offset_y);
                true
            },
            GameMsg::KeyDown(_key) => {
                
                true
            },
            GameMsg::KeyUp(_key) => {
                true
            },
            GameMsg::Update => {
                self.game_update();
                false
            }
            GameMsg::Render => {
                // self.render();
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
        // Only start the render loop if it's the first render
        // There's no loop cancellation taking place, so if multiple renders happen,
        // there would be multiple loops running. That doesn't *really* matter here because
        // there's no props update and no SSR is taking place, but it is something to keep in
        // consideration
        // self.game_update();

        if !first_render {
            return;
        }
        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.

        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();
        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

      
        self.render_gl(gl, ctx);
    }
}

// Code taken from https://github.com/yewstack/yew/blob/master/examples/webgl/src/main.rs 
impl GameControl {

    fn game_update(&mut self) {
        let cur_time = Date::now();
        let diff = cur_time - self.last_update;

        let frac = diff; // / 1000.0; // Fraction of a second
        self.rotation += diff;

        self.cur_time = cur_time;
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn render_gl(&mut self, gl: GL, ctx: &Context<GameControl>) {
        // This should log only once -- not once per frame

        let mut timestamp = 0.0;

        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");

        // This list of vertices will draw two triangles to cover the entire canvas.
        // let vertices: Vec<f32> = vec![
        //     -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        // ];

        let vertices: Vec<f32> = vec![
           -0.5,0.5,0.0,
            -0.5,-0.5,0.0,
            0.5,-0.5,0.0,
        ];
        // let indices = [0,1,2];
        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);
        gl.bind_buffer(GL::ARRAY_BUFFER, None);

        // let index_buffer = gl.create_buffer().unwrap();
        // let inds = js_sys::Uint16Array::from(indices.as_slice());
        // gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        // gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &inds, GL::STATIC_DRAW);
        // gl.bind_buffer(GL::ARRAY_BUFFER, None);

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

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer) );
        // gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&shader_program, "coords") as u32;
        gl.vertex_attrib_pointer_with_f64(position, 3, GL::FLOAT, false, 0, 0.0);
        gl.enable_vertex_attrib_array(position);

        // Attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);

        let mut rot = self.rotation;
        let rot_pos = gl.get_uniform_location(&shader_program, "rot");
        gl.uniform1f(rot_pos.as_ref(), rot as f32);

        gl.viewport(0, 0, GAME_WIDTH as i32, GAME_HEIGHT as i32);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        gl.draw_arrays(GL::TRIANGLES, 0, 3);

        // Gloo-render's request_animation_frame has this extra closure
        // wrapping logic running every frame, unnecessary cost.
        // Here constructing the wrapped closure just once.

        let cb = Rc::new(RefCell::new(None));
        let comp_ctx = ctx.link().clone();
        // let rot_ref = &self.rotation;
        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = cb.clone();
            move || {
                comp_ctx.send_message(GameMsg::Update);

                
                // let rot_pos = gl.get_uniform_location(&shader_program, "rot");
                rot += 0.01;
                if rot > 2.0 {
                    rot = 0.01;
                }
                gl.uniform1f(rot_pos.as_ref(), rot as f32);
                
                // This should repeat every frame
                timestamp += 20.0;
                gl.uniform1f(time.as_ref(), timestamp as f32);
                gl.draw_arrays(GL::TRIANGLES, 0, 3);
                GameControl::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));

        GameControl::request_animation_frame(cb.borrow().as_ref().unwrap());
    }

    // fn game_update(&mut self) {
    //     let cur_time = Date::now();
    //     let diff = cur_time - self.last_update;
        
    //     self.cur_time += diff;

    //     self.last_update = cur_time;

    //     self.mouse.update(diff);
        
    // }

    // fn render(&mut self) {

    //     self.game_update();

    //     let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        
    //     // Make sure the we reset the draw surface to prevent stretching
    //     canvas.set_width(canvas.client_width() as u32);
    //     canvas.set_height(canvas.client_height() as u32);

    //     let mut gl: WebGlRenderingContext =
    //         canvas.get_context("webgl").unwrap().unwrap().unchecked_into();

    //     gl.clear_color(0.0, 0.0, 0.0, 1.0);
    //     // Clear the color buffer with specified clear color
    //     gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    //     window()
    //         .unwrap()
    //         .request_animation_frame(self.callback.as_ref().unchecked_ref())
    //         .unwrap();
    // }

}

// pub fn init_webgl_context(canvas_id: &str) -> Result<WebGlRenderingContext, JsValue> {
//     let document = web_sys::window().unwrap().document().unwrap();
//     let canvas = document.get_element_by_id(canvas_id).unwrap();
//     let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
//     let gl: WebGlRenderingContext = canvas
//         .get_context("webgl")?
//         .unwrap()
//         .dyn_into::<WebGlRenderingContext>()
//         .unwrap();

//     gl.viewport(
//         0,
//         0,
//         canvas.width().try_into().unwrap(),
//         canvas.height().try_into().unwrap(),
//     );

//     Ok(gl)
// }


// pub fn setup_vertices(gl: &WebGlRenderingContext, vertices: &[f32], shader_program: &WebGlProgram) {
//     let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };
//     let vertex_buffer = gl.create_buffer().unwrap();

//     gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
//     gl.buffer_data_with_array_buffer_view(
//         WebGlRenderingContext::ARRAY_BUFFER,
//         &vertices_array,
//         WebGlRenderingContext::STATIC_DRAW,
//     );

//     let coordinates_location = gl.get_attrib_location(&shader_program, "coordinates");

//     gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
//     gl.vertex_attrib_pointer_with_i32(
//         coordinates_location as u32,
//         3,
//         WebGlRenderingContext::FLOAT,
//         false,
//         0,
//         0,
//     );
//     gl.enable_vertex_attrib_array(coordinates_location as u32);
// }

// pub fn setup_shaders(gl: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {
//     let vertex_shader_source = "
//         attribute vec3 coordinates;
//         void main(void) {
//             gl_Position = vec4(coordinates, 1.0);
//         }
//         ";

//     let fragment_shader_source = "
//         precision mediump float;
//         uniform vec4 fragColor;
//         void main(void) {
//             gl_FragColor = fragColor;
//         }
//         ";

//     let vertex_shader = create_shader(
//         &gl,
//         WebGlRenderingContext::VERTEX_SHADER,
//         vertex_shader_source,
//     )
//     .unwrap();
//     let fragment_shader = create_shader(
//         &gl,
//         WebGlRenderingContext::FRAGMENT_SHADER,
//         fragment_shader_source,
//     )
//     .unwrap();

//     let shader_program = gl.create_program().unwrap();
//     gl.attach_shader(&shader_program, &vertex_shader);
//     gl.attach_shader(&shader_program, &fragment_shader);
//     gl.link_program(&shader_program);

//     if gl
//         .get_program_parameter(&shader_program, WebGlRenderingContext::LINK_STATUS)
//         .as_bool()
//         .unwrap_or(false)
//     {
//         gl.use_program(Some(&shader_program));
//         Ok(shader_program)
//     } else {
//         return Err(JsValue::from_str(
//             &gl.get_program_info_log(&shader_program)
//                 .unwrap_or_else(|| "Unknown error linking program".into()),
//         ));
//     }
// }

// pub fn create_shader(
//     gl: &WebGlRenderingContext,
//     shader_type: u32,
//     source: &str,
// ) -> Result<WebGlShader, JsValue> {
//     let shader = gl
//         .create_shader(shader_type)
//         .ok_or_else(|| JsValue::from_str("Unable to create shader object"))?;

//     gl.shader_source(&shader, source);
//     gl.compile_shader(&shader);

//     if gl
//         .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
//         .as_bool()
//         .unwrap_or(false)
//     {
//         Ok(shader)
//     } else {
//         Err(JsValue::from_str(
//             &gl.get_shader_info_log(&shader)
//                 .unwrap_or_else(|| "Unknown error creating shader".into()),
//         ))
//     }
// }

// #[wasm_bindgen]
// pub fn draw_triangle(
//     canvas_id: &str,
//     selected_color: Option<Vec<f32>>,
// ) -> Result<WebGlRenderingContext, JsValue> {
//     let gl: WebGlRenderingContext = init_webgl_context(canvas_id).unwrap();
//     let shader_program: WebGlProgram = setup_shaders(&gl).unwrap();
//     let vertices: [f32; 9] = [
//         0.0, 1.0, 0.0, // top
//         -1.0, -1.0, 0.0, // bottom left
//         1.0, -1.0, 0.0, // bottom right
//     ];

//     setup_vertices(&gl, &vertices, &shader_program);

//     let color = selected_color.unwrap_or(vec![1.0, 0.0, 0.0, 1.0]);
//     let color_location = gl
//         .get_uniform_location(&shader_program, "fragColor")
//         .unwrap();
//     gl.uniform4fv_with_f32_array(Some(&color_location), &color);

//     gl.draw_arrays(
//         WebGlRenderingContext::TRIANGLES,
//         0,
//         (vertices.len() / 3) as i32,
//     );

//     Ok(gl)
// }