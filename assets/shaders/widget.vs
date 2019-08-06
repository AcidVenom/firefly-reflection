#version 330

in vec2 position;
in vec2 uv;

out vec2 UV;

uniform mat4 model;
uniform mat4 projection;

void main() 
{
    gl_Position = projection * model * vec4(position, 1.0, 1.0);
    UV = uv;
}