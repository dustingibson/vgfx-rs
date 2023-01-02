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
uniform vec3 rotate;

mat3 xrotate(float angle) {
	return mat3(
		1.0f, 0.0f, 0.0f,
		0.0f, cos(angle), -1.0f*sin(angle),
		0.0f, sin(angle), cos(angle)
	);
}

mat3 yrotate(float angle) {
	return mat3(
		cos(angle), 0.0f, sin(angle),
		0.0f, 1.0f, 0.0f,
		-1.0f*sin(angle), 0.0f, cos(angle)
	);
}

mat3 zrotate(float angle) {
	return mat3(
		cos(angle), -1.0f*sin(angle), 0.0f,
		sin(angle), cos(angle), 0.0f,
		0.0f, 0.0f, 1.0f
	);
}

void main() {
	vec3 new_pos = vec3(scale.x * model_pos.x, scale.y * model_pos.y, scale.z * model_pos.z);
	new_pos = new_pos*xrotate(rotate.x);
	new_pos = new_pos*yrotate(rotate.y);
	new_pos = new_pos*zrotate(rotate.z);
	gl_Position = projection * view * model * vec4(new_pos, 1.0f);
	fragPos = vec3(model * vec4(new_pos,1.0f));
	normal = normals;
	TexCoord = aTexCoord;
}