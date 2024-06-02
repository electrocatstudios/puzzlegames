precision mediump float;

attribute vec3 coords;
uniform float u_time;
uniform float rot; 

void main() {
    float x = coords[0] + (sin(u_time / 1000.0) * 0.1);
    float y = coords[1] + rot;
    float z = coords[2] + rot;

    gl_Position = vec4(x, y , z, 1.0);
    // gl_Position = a_position;
}