use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point {
            x: x,
            y: y
        }
    }
}

pub struct Rect<T> {
    pub loc: Point<T>,
    pub width: T,
    pub height: T
}

impl<T> Rect<T> {
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Rect {
            loc: Point::new(x,y),
            width: w,
            height: h
        }
    }
}

pub struct Circle<T> {
    pub loc: Point<T>,
    pub size: T,
}

impl<T> Circle<T> {
    pub fn new(x: T, y: T, size: T) -> Self {
        Circle {
            loc: Point::new(x,y),
            size: size
        }
    }
}

pub fn format_time_3_digits(time: f64) -> String {
    if time < 10.0 {
        format!("00{}", time)
    } else if time < 100.0 {
        format!("0{}", time)
    } else if time < 1000.0 {
        time.to_string()
    } else {
        // If they took longer than 1000s then don't
        // let them see the full time - string too big!
        "999".to_string()
    }
}

pub fn format_time_2_digits(time: f64) -> String {
    // Trim to 2 most significant chars
    let out_str = format_time_3_digits(time);
    out_str[1..].to_string()
}

pub fn drop_shadow_string(ctx: &mut CanvasRenderingContext2d, text: String, x: f64, y: f64) {
    ctx.set_fill_style(&JsValue::from("rgb(0,0,0)"));
    let _ = ctx.fill_text(&text, x + 3.0, y+3.0);
    ctx.set_fill_style(&JsValue::from("rgb(255,0,0)"));
    let _ = ctx.fill_text(&text, x, y);
}