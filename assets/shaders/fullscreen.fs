#version 330

in vec2 UV;

out vec4 output0;

uniform float time;
uniform sampler2D sampler0;

void main()
{
    output0 = texture(sampler0, UV);
}