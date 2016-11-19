#version 150 core

in vec2 v_TexCoord;
in float v_SideWidth;
out vec4 Target0;

uniform sampler2D t_Color;
uniform Locals {
	mat4 u_Transform;
};

void main() {
    vec2 adjusted_TexCoord = v_TexCoord * v_SideWidth;
    adjusted_TexCoord = mod(adjusted_TexCoord, 1.0);

    vec4 tex = texture(t_Color, adjusted_TexCoord);
    float blend = dot(adjusted_TexCoord-vec2(0.5,0.5),
                      adjusted_TexCoord-vec2(0.5,0.5));
    Target0 = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
}
