#version 330 core

in vec3 normal;
in vec3 fragPos;
in vec2 TexCoord;
out vec4 color;

uniform vec4 ambientColor;
uniform sampler2D textureSample;
uniform int textured;

void main() {
	// vec3 lightPos = vec3(0.0f, 0.0f, 0.0f);
	// vec3 norm = normalize(normal);
	// vec3 lightDir = normalize(lightPos - fragPos);
	// vec4 lightColor = vec4(0.99f, 0.99f, 0.83f, 1.0f);
	// vec4 ambient = 0.01f * lightColor;
	// float diff = max(dot(norm, lightDir), 0.0);
	// vec4 diffuse = diff * lightColor;
	if(textured > 0) {
		// By Texture Segment
		color = texture(textureSample, TexCoord).rgba;
	}
	else {
		color = ambientColor;
	}
	//color = result;
}