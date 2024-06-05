precision mediump float;

attribute vec4 a_position;
uniform mat4 u_matrix;

varying vec4 v_positionWithOffset;

void main() {
    gl_Position = u_matrix * a_position;
    v_positionWithOffset = a_position;
}