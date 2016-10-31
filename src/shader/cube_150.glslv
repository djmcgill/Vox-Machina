#version 150 core

in vec4 a_Pos;
in vec2 a_TexCoord;
in vec3 a_Translate;
out vec2 v_TexCoord;

uniform Locals {
	mat4 u_Transform;
};

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = u_Transform * (a_Pos + vec4(a_Translate, 1.0));
    gl_ClipDistance[0] = 1.0;
}
