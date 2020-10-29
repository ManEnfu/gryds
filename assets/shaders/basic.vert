#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 2) in vec3 Normal;
layout (location = 3) in vec3 Ambient;
layout (location = 4) in vec3 Diffuse;
layout (location = 5) in vec3 Specular;

uniform mat4 Model;
uniform mat4 MVP;
uniform mat4 MV;
uniform vec3 lightSource;
uniform vec3 viewer;
uniform int useShading;

out vec3 vColor;
out vec3 vPos;
void main()
{
    vec3 modelPos = (Model * vec4(Position, 1.0)).xyz;
    vec3 mNormal = (Model * vec4(Normal, 1.0)).xyz;
    vec3 viewPos = (MV * vec4(Position, 1.0)).xyz;
    vec3 vNormal = (MV * vec4(Normal, 1.0)).xyz;

    vec3 h = normalize(viewer + lightSource - 2 * modelPos);
    gl_Position = MVP * vec4(Position, 1.0);

    vPos = Position;
    if (useShading == 1) {    
        vColor = Diffuse * 1.5 * clamp(dot(normalize(lightSource - modelPos), mNormal), 0.0, 1.0)
            + Ambient * 0.1;
    } else {
        vColor = Diffuse;
    }
}
