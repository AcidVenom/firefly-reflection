#version 330

in vec2 UV;

out vec4 color;
uniform vec4 blend;
uniform sampler2D sampler0;

void main()
{
    vec2 uv = max(vec2(0.01), min(UV, vec2(0.99)));
    color = texture(sampler0, uv) * blend;
}