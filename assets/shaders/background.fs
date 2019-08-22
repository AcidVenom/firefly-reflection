#version 330

in vec2 UV;

out vec4 output0;

uniform float time;
uniform mat4 view;
uniform mat4 projection;
uniform vec4 blend;

#define DITTER_DIVIDER 128.0

// Taken from http://enbseries.enbdev.com/forum/viewtopic.php?f=7&t=5220
vec3 dither(vec2 screen_pos, vec3 color)
{
    vec3 magic = vec3(0.06711056, 0.00583715, 52.9829189);
    return color + mix(-0.5, 0.5, fract(magic.z * fract(dot(screen_pos, magic.xy)))) / DITTER_DIVIDER;
}

vec3 srgb(vec3 color)
{
    return pow(color, vec3(1.0 / 2.2));
}

// Taken from https://gist.github.com/patriciogonzalezvivo/670c22f3966e662d2f83
vec3 permute(vec3 x)
{ 
    return mod(((x*34.0)+1.0)*x, 289.0); 
}

float snoise(vec2 v)
{
    const vec4 C = vec4(0.211324865405187, 0.366025403784439, -0.577350269189626, 0.024390243902439);
    vec2 i  = floor(v + dot(v, C.yy) );
    vec2 x0 = v -   i + dot(i, C.xx);
    vec2 i1;
    i1 = (x0.x > x0.y) ? vec2(1.0, 0.0) : vec2(0.0, 1.0);
    vec4 x12 = x0.xyxy + C.xxzz;
    x12.xy -= i1;
    i = mod(i, 289.0);
    vec3 p = permute( permute( i.y + vec3(0.0, i1.y, 1.0 ))
    + i.x + vec3(0.0, i1.x, 1.0 ));
    vec3 m = max(0.5 - vec3(dot(x0,x0), dot(x12.xy,x12.xy),
    dot(x12.zw,x12.zw)), 0.0);
    m = m*m ;
    m = m*m ;
    vec3 x = 2.0 * fract(p * C.www) - 1.0;
    vec3 h = abs(x) - 0.5;
    vec3 ox = floor(x + 0.5);
    vec3 a0 = x - ox;
    m *= 1.79284291400159 - 0.85373472095314 * ( a0*a0 + h*h );
    vec3 g;
    g.x  = a0.x  * x0.x  + h.x  * x0.y;
    g.yz = a0.yz * x12.xz + h.yz * x12.yw;
    return 130.0 * dot(m, g);
}

vec3 gradient(vec2 screen_pos, vec2 screen_size)
{
    const float halo_offset = 50.0;
    float world_y_pos = -view[3][1] + screen_pos.y;

    float y_pos = (world_y_pos + halo_offset) / screen_size.y;
    y_pos = max(0.0, min(y_pos, 1.0));

    vec3 halo_color = vec3(18.0 / 255.0, 29.0 / 255.0, 39.0 / 255.0);
    vec3 sky_ramp = mix(halo_color, vec3(0.0), y_pos);

    return dither(screen_pos, sky_ramp);
}

vec3 rain(vec3 base, vec2 screen_pos)
{
    vec2 world_pos = screen_pos - vec2(view[3][0], view[3][1]);
    vec2 rain_pos = (world_pos + vec2(0.0, time * 1050.0)) * vec2(0.175, 0.0075);
    float noise = min(max(0.0, snoise(rain_pos) * 2.5 - mix(2.5, 1.65, blend.a)), 1.0);

    return mix(base, vec3(0.0), noise);
}

void main()
{
    const vec2 screen_size = vec2(1280.0, 720.0);

    vec2 screen_pos = UV * screen_size;
    
    vec3 result = gradient(screen_pos, screen_size);
    result = rain(result, screen_pos);

    output0 = vec4(srgb(result), 1.0);
}