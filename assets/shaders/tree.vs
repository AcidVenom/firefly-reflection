#version 330

in vec2 position;
in vec2 uv;

out vec2 UV;

uniform float time;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() 
{
    mat4 parallax = view;
    parallax[3][0] = (parallax[3][0] - 1280.0) * model[0][0] / 600.0;
    vec4 world_pos = model * vec4(position, 0.0, 1.0);
    vec2 pos = position;
    float wind_factor = uv.y * sin(time * 3.0 + sin(world_pos.x * 0.01));
    pos.x += wind_factor * 0.033;
    pos.y += pow(abs(wind_factor), 4.0) * 0.01;
    gl_Position = projection * parallax * model * vec4(pos, 0.0, 1.0);
    UV = uv;
}