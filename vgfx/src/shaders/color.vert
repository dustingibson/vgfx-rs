#version 330 core

layout(location = 0) in vec3 model_pos;
layout(location = 1) in vec4 vertex_color;

out vec4 fragmentColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {	
	gl_Position = projection * view * model * vec4(model_pos, 1.0f);
	fragmentColor = vertex_color;
}