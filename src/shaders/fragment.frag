#version 330 core

in vec3 fragmentColor;
in vec3 normal;
in vec3 fragPos;
out vec3 color;

uniform vec3 lightPos;

void main() {
	vec3 norm = normalize(normal);
	vec3 lightDir = normalize(lightPos - fragPos);
	vec3 lightColor = vec3(1.0f, 1.0f, 1.0f);
	vec3 ambient = 0.1f * lightColor;
	float diff = max(dot(norm, lightDir), 0.0);
	vec3 diffuse = diff * lightColor;
	vec3 result = (ambient + diffuse) * fragmentColor;
	color = result;

	//color = fragmentColor;
}