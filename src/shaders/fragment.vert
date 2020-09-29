#version 330 core

layout(location = 0) in vec3 model_pos;
layout(location = 1) in vec3 vertex_color;
layout(location = 2) in vec3 normals;

out vec3 fragmentColor;
out vec3 normal;
out vec3 fragPos;

uniform mat4 MVP;

void main() {	
	gl_Position =  MVP * vec4(model_pos,1);
	fragPos = model_pos;
	fragmentColor = vertex_color;
	normal = normals;
}