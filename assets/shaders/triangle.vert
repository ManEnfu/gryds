#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

uniform mat4 MVP;

out vec3 vColor;
out vec3 vPos;
void main()
{
    gl_Position = MVP * vec4(Position, 1.0);
    vPos = Position;
    vColor = Color;
}
