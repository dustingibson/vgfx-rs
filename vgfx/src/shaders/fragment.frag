#version 330 core

in vec3 normal;
in vec4 fragmentColor;
in vec3 fragPos;
in vec2 TexCoord;
out vec4 color;

uniform vec3 lightPos;
uniform sampler2D textureSample;
uniform int textured;

void main() {
	vec3 norm = normalize(normal);
	vec3 lightDir = normalize(lightPos - fragPos);
	vec4 lightColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
	vec4 ambient = 0.1f * lightColor;
	float diff = max(dot(norm, lightDir), 0.0);
	vec4 diffuse = diff * lightColor;
	if(textured > 0) {
		color = texture(textureSample, TexCoord).rgba;
		//color = (ambient + diffuse) * texture(textureSample, TexCoord).rgba;
		color.a = texture(textureSample, TexCoord).a;
	}
	else {
		//color = (ambient + diffuse) * fragmentColor.rgba;
		color = fragmentColor.rgba;
		color.a = fragmentColor.a;
	}
	//color = result;
}