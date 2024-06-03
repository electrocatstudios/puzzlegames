precision mediump float;

attribute vec3 coords;

uniform float x;
uniform float y;
uniform float z;

uniform float u_time;
uniform float rot; 

void main() {
    float x_pos = (coords[0] + x) / (1280.0 / 2.0);
    float y_pos = (coords[1] + y) / (800.0 / 2.0) ;
    float z_pos = (coords[2] + z) ;

    gl_Position = vec4(x_pos, y_pos , z_pos, 1.0);
}