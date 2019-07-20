#version 330

in vec2 UV;

out vec4 diffuse;

uniform sampler2D sampler0;

void main()
{
    diffuse = texture(sampler0, UV);
}