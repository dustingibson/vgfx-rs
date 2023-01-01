#version 330 core

layout(location = 0) in vec3 model_pos;
layout(location = 1) in vec3 normals;
layout(location = 2) in vec2 aTexCoord;

out vec3 normal;
out vec3 fragPos;
out vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 scale;

void main() {
	vec3 new_pos = vec3(scale.x * model_pos.x, scale.y * model_pos.y, scale.z * model_pos.z);
	gl_Position = projection * view * model * vec4(new_pos, 1.0f);
	fragPos = vec3(model * vec4(new_pos,1.0f));
	normal = normals;
	TexCoord = aTexCoord;
}