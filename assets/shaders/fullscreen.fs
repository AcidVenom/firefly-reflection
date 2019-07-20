#version 330

in vec2 UV;

out vec4 diffuse;

uniform float time;
uniform sampler2D sampler0;

void main()
{
    vec2 uv = vec2(UV.x, 1.0 - UV.y);
    uv.y += sin(uv.x * 10.0 + time) * 0.1;
    diffuse = texture(sampler0, uv);
}