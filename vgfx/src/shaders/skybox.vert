#version 330 core

layout(location = 0) in vec3 model_pos;


out vec3 TexCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {	
	gl_Position = projection * view * model * vec4(model_pos, 1.0f);
	TexCoords = model_pos;
}