#version 130

in vec2 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

uniform mat4 matrix;
uniform vec2 screen_dimensions;
uniform vec2 offset;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = vec4(
        -1.0 + ((position.x + offset.x) / screen_dimensions.x) * 2,
        1.0 - ((position.y + offset.y) / screen_dimensions.y) * 2,
        0.0,
        1.0
    );
}