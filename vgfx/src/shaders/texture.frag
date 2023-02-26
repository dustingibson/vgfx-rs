#version 330 core

in vec3 normal;
in vec3 fragPos;
in vec2 TexCoord;
out vec4 color;

uniform vec4 ambientColor;
uniform vec3 lightPos;
uniform sampler2D textureSample;
uniform sampler2D lightSample;
uniform int textured;

void main() {
	vec4 new_color = vec4(0.0f, 0.0f, 0.0f, 0.0f);
	if(textured > 0) {
		// By Texture Segment
		new_color = texture(textureSample, TexCoord).rgba;
	}
	else {
		new_color = ambientColor;
	}

	color = new_color;
}