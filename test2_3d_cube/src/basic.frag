precision mediump float;

uniform float u_time;
uniform float rot; 

varying vec4 v_positionWithOffset;

void main() {
    float r = 0.2 + (0.8 * (v_positionWithOffset[0]/70.0));
    float g = 0.2 + (0.8 * (v_positionWithOffset[1]/70.0));
    float b = 0.2 + (0.8 * (v_positionWithOffset[2]/70.0));

    gl_FragColor = vec4(r, g, b, 1.0);
}