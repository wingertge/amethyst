
#version 450

layout(set = 1, binding = 0) uniform sampler2D tex;
layout(set = 2, binding = 0) uniform UiBoundsArgs {
    vec2 min;
    vec2 max;
} bounds;

layout(location = 0) in vec2 in_tex_coords;
layout(location = 1) in vec4 in_color;
layout(location = 2) in vec4 in_color_bias;

layout(location = 0) out vec4 out_color;

void main() {
    vec2 pos = vec2(gl_FragCoord);
    if(pos.x < bounds.min.x || pos.y < bounds.min.y || pos.x > bounds.max.x || pos.y > bounds.max.y) {
        discard;
    }

    vec4 color = (texture(tex, in_tex_coords) + in_color_bias) * in_color;
    if (color.a == 0.0) {
        discard;
    }

    out_color = color;
} 