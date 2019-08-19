#version 330

in vec2 UV;

out vec4 color;
uniform vec4 blend;
uniform sampler2D sampler0;

void main()
{
    color = texture(sampler0, UV) * blend;
    color.rgb = pow(color.rgb, vec3(1.0 / 2.2));
}