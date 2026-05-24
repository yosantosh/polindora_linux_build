#version 440

layout(location = 0) in vec2 qt_TexCoord0;
layout(location = 0) out vec4 fragColor;

layout(std140, binding = 0) uniform buf {
    mat4 qt_Matrix;
    float qt_Opacity;
    float pulse;
};

void main() {
    vec2 uv = qt_TexCoord0 * 2.0 - 1.0;
    float dist = length(uv);
    float rim = smoothstep(0.98, 0.74, dist) - smoothstep(1.0, 0.96, dist);
    float highlight = smoothstep(0.55, 0.0, distance(uv, vec2(-0.34, -0.42)));
    vec3 base = vec3(0.02, 0.08, 0.19);
    vec3 glow = vec3(0.32, 0.62, 1.0) * (rim + highlight * 0.55 + pulse * 0.08);
    float alpha = smoothstep(1.0, 0.92, dist) * qt_Opacity;
    fragColor = vec4(base + glow, alpha);
}
