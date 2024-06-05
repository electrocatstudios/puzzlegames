precision mediump float;

uniform float u_time;
uniform float rot; 

varying vec4 v_positionWithOffset;

void main() {
    float r = 1.0 * (v_positionWithOffset[0]/50.0);
    float g = 1.0 * (v_positionWithOffset[1]/50.0);
    float b = 1.0 * (v_positionWithOffset[2]/50.0);

    gl_FragColor = vec4(r, g, b, 1.0);
}