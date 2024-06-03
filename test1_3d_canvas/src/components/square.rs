use web_sys::WebGlProgram;
use crate::utils::Point3;

pub struct Square {
    pub loc: Point3<f64>,
    pub vertices: Vec::<f32>,
    pub shader: Option<WebGlProgram>
}

impl Square {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let verts = vec![
            -50.0, 50.0, 0.0,
            50.0, 50.0, 0.0,
            -50.0, -50.0, 0.0,
            -50.0, -50.0, 0.0,
            50.0, 50.0, 0.0,
            50.0, -50.0, 0.0
        ];
        Square {
            loc: Point3::new(x,y,z),
            vertices: verts,
            shader: None
        }
    }
}