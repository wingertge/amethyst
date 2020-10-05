
#version 450

layout(set = 1, binding = 0) uniform sampler2D tex;

layout(location = 0) in vec2 in_tex_coords;
layout(location = 1) in vec4 in_color;
layout(location = 2) in vec4 in_color_bias;
layout(location = 3) in vec2 bounds_min;
layout(location = 4) in vec2 bounds_max;

layout(location = 0) out vec4 out_color;

void main() {
    vec2 pos = gl_FragCoord.xy;
    if(pos.x < bounds_min.x || pos.y < bounds_min.y || pos.x > bounds_max.x || pos.y > bounds_max.y) {
        discard;
    }

    vec4 color = (texture(tex, in_tex_coords) + in_color_bias) * in_color;
    if (color.a == 0.0) {
        discard;
    }

    out_color = color;
} 