use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use std::collections::HashMap;

pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3 {
            x: x,
            y: y,
            z: z
        }
    }
}

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point {
            x: x,
            y: y,
        }
    }
}


pub fn is_key_pressed(keys: &HashMap<String,bool>, key: &String) -> bool {
    match keys.get(key) {
        Some(val) => {
            *val
        },
        None => false
    } 
}
