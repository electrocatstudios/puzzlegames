precision mediump float;

uniform float u_time;
uniform float rot; 

void main() {
    float r = 0.2 + (sin(rot / 1000.0) / 2.0); //sin(u_time * 0.0003);
    float g = 0.2 - (sin(rot / 1000.0) / 2.0); // sin(u_time * 0.0005);
    float b = 0.9;

    gl_FragColor = vec4(r, g, b, 1.0);
}