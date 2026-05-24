#version 440

layout(location = 0) in vec2 qt_TexCoord0;
layout(location = 0) out vec4 fragColor;

layout(std140, binding = 0) uniform buf {
    mat4 qt_Matrix;
    float qt_Opacity;
    float intensity;
};

void main() {
    vec2 uv = qt_TexCoord0;
    float edge = smoothstep(0.0, 0.12, uv.y) * (1.0 - smoothstep(0.72, 1.0, uv.y));
    float sheen = smoothstep(0.0, 0.5, 1.0 - distance(uv, vec2(0.26, 0.08)) * 2.2);
    vec3 color = mix(vec3(0.05, 0.10, 0.20), vec3(0.54, 0.72, 1.0), sheen * intensity);
    fragColor = vec4(color, (0.20 + edge * 0.18) * qt_Opacity);
}
