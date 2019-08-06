#version 330

in vec2 UV;

out vec4 color;
uniform sampler2D sampler0;

void main()
{
    color = texture(sampler0, UV);
}