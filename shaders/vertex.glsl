#version 140

// Base mesh attributes
in vec3 pos;
in vec3 base_color; // Base mesh color (if needed)

// Instance attributes (per particle)
in vec3 i_pos;
in vec3 i_color;
in float i_radius;

out vec3 v_color;
uniform mat4 matrix;

void main() {
    // Transform the base vertex position for this instance
    vec3 world_pos = pos * i_radius + i_pos;

    v_color = i_color;
    gl_Position = matrix * vec4(world_pos, 1.0);
}
