#version 330 core

out vec4 color;
in vec3 TexCoords;
uniform samplerCube textureSample;


void main() {
	color = texture(textureSample, TexCoords).rgba;
}