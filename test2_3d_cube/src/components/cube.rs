use web_sys::WebGlProgram;
use crate::utils::Point3;

pub struct Cube {
    pub rot: Point3<f64>,
    pub loc: Point3<f64>,
    pub vertices: Vec::<f32>,
    pub shader: Option<WebGlProgram>
}

impl Cube {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let verts = vec![
            // Front face
            -50.0, 50.0, 50.0,
            -50.0, -50.0, 50.0,
            50.0, 50.0, 50.0,
            -50.0, -50.0, 50.0,
            50.0, -50.0, 50.0,
            50.0, 50.0, 50.0,
            // Back face
            -50.0, 50.0, -50.0,
            50.0, 50.0, -50.0,
            -50.0, -50.0, -50.0,
            -50.0, -50.0, -50.0,
            50.0, 50.0, -50.0,
            50.0, -50.0, -50.0,
            // Left Side
            -50.0, 50.0, -50.0,
            -50.0, 50.0, 50.0,
            -50.0, -50.0, -50.0,
            -50.0, -50.0, -50.0,
            -50.0, 50.0, 50.0,
            -50.0, -50.0, 50.0,
            // Right Side
            50.0, 50.0, -50.0,
            50.0, -50.0, -50.0,
            50.0, 50.0, 50.0,
            50.0, -50.0, -50.0,
            50.0, -50.0, 50.0,
            50.0, 50.0, 50.0,
            // Top Side
            -50.0, 50.0, -50.0,
            50.0, 50.0, -50.0,
            -50.0, 50.0, 50.0,
            -50.0, 50.0, 50.0,
            50.0, 50.0, -50.0,
            50.0, 50.0, 50.0,
            // Bottom Side
            -50.0, -50.0, -50.0,
            -50.0, -50.0, 50.0,
            50.0, -50.0, -50.0,
            -50.0, -50.0, 50.0,
            50.0, -50.0, 50.0,
            50.0, -50.0, -50.0,
            
        ];
        Cube {
            rot: Point3::new(0.0,0.0,0.0),
            loc: Point3::new(x,y,z),
            vertices: verts,
            shader: None
        }
    }
}