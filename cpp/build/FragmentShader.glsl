#version 330 core

// Input data
in vec2 UV;

// Ouput data
out vec3 color;

void main(){
	// Output color = color in vertex shader
	color = texture(myTextureSampler, UV).rgb;
}
