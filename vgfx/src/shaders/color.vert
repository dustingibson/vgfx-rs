#version 330 core

layout(location = 0) in vec3 model_pos;
layout(location = 1) in vec3 normals;
layout(location = 2) in vec2 aTexCoord;

//out vec3 fragPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {	
	gl_Position = projection * view * model * vec4(model_pos, 1.0f);
	//fragPos = vec3(model * vec4(model_pos,1.0f));
}