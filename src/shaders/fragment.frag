#version 330 core

in vec3 normal;
in vec4 fragmentColor;
in vec3 fragPos;
out vec4 color;

uniform vec3 lightPos;

void main() {
	vec3 norm = normalize(normal);
	vec3 lightDir = normalize(lightPos - fragPos);
	vec4 lightColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
	vec4 ambient = 0.1f * lightColor;
	float diff = max(dot(norm, lightDir), 0.0);
	vec4 diffuse = diff * lightColor;
	vec4 result = (ambient + diffuse) * fragmentColor;
	result.a = fragmentColor.w;
	color = result;
	//color = fragmentColor;
}