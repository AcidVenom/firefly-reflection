#version 330

in vec2 UV;

out vec4 diffuse;

uniform sampler2D tex0;

void main()
{
    diffuse = texture(tex0, UV);
}