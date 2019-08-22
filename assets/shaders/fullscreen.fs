#version 330

in vec2 UV;

out vec4 output0;

uniform float time;
uniform vec4 blend;
uniform sampler2D sampler0;
uniform mat4 view;

void main()
{
    // I call this, "the-arbritrary-numbers-until-it-looks-good" shader
    // .. For the "lake" reflection, by the way
    const vec2 screen_size = vec2(1280.0, 720.0);
    vec2 screen_pos = UV * screen_size;
    vec2 world_pos = -vec2(view[3][0], view[3][1]) + vec2(screen_pos.x, screen_pos.y);

    vec4 base = texture(sampler0, UV);

    vec2 distort = vec2(pow(sin(world_pos.y * 0.15 + time * 1.0), 3.0), 0.0) * 0.004;

    float camera_offset = -200.0 - view[3][1];

    vec2 sample_coord = vec2(0.0, 0.42 + camera_offset / 360.0) + distort;
    sample_coord = (vec2((UV.x + 0.005) * 0.99, UV.y) + sample_coord) * vec2(1.0, -1.0);
    vec4 lake = texture(sampler0, sample_coord);

    vec3 lake_color = vec3(0.05, 0.1, 0.15);
    lake.rgb = mix(lake.rgb, lake_color, 0.4);

    vec4 result = world_pos.y > 40.0 + sin(world_pos.x / 30.0 + time * 1.0) * cos(world_pos.x / 300.0 + time * 3.0) * 2.0 ? base : lake;

    // End of lake stuff..

    // Vignette

    float d = max(0.0, min(1.0 - length(abs((UV - vec2(0.0, -0.1)) * 2.0 - 1.0)) * (1.0 - UV.y) * 0.5, 1.0));
    result.rgb = mix(vec3(0.0, 0.025, 0.1), result.rgb, d);
    
    output0 = result;

    // Fading
    output0.rgb *= blend.a;
}