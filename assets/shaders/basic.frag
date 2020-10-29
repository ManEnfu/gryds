#version 330 core

in vec3 vColor;
in vec3 vPos;

out vec4 Color;

void main()
{
    Color = vec4(vColor, 1.0f);
}
