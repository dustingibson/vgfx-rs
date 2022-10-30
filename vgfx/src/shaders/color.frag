#version 330 core

out vec4 color;

uniform vec4 ambientColor;
uniform sampler2D textureSample;
uniform int textured;

void main() {
	color = vec4(0.0, 1.0, 0.0, 1.0);
}