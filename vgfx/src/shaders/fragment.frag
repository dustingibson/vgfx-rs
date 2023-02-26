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
	vec3 normFragPos = normalize(fragPos);
	vec4 texture_color = textured > 0 ? texture(textureSample, TexCoord).rgba : ambientColor;
	vec3 norm = normalize(normal);
	vec3 light_dir = normalize(-vec3(0.7f, -0.6f, 0.2f));
	
	vec4 light_color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
	//vec4 lightColor = texture(lightSample, vec2(normFragPos.x, normFragPos.y) ).rgba;
	
	float diff = max(dot(norm, light_dir), 0.0);
	vec4 diffuse_color = vec4(0.5f, 0.5f, 0.5f, 1.0f);
	vec4 diffuse = diffuse_color * diff * texture_color;

	vec4 ambient_color = vec4(0.2f, 0.2f, 0.2f, 1.0f);
    vec4 ambient = ambient_color * texture_color;

	color = (diffuse + ambient);
	color = vec4(color.x, color.y, color.z, 1.0);
	
	//color = new_color;
}