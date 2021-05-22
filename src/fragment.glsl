#version 140

out vec4 color;

in vec2 v_uv;

uniform sampler2D noise_text;

// Uniform values
uniform float time;
uniform vec2 resolution;
uniform vec3 eye;

// Ray marching configuration
const int MAX_MARCHING_STEPS = 255;
const float MIN_DIST = 0.0;
const float MAX_DIST = 100.0;
const float EPSILON = 0.0001;

/**
 * Intersection of the result from two different SDF.
 *
 * Returns the maximum distance between the two.
 */
float intersectSDF(float a, float b) {
    return max(a, b);
}

/**
 * Union of the result from two different SDF.
 *
 * Returns the minimum distance between the two.
 */
float unionSDF(float a, float b) {
    return min(a, b);
}

/**
 * Diference between two SDF.
 */
float differenceSDF(float a, float b) {
    return max(a, -b);
}

/**
 * SDF for a segment that start in a and ends in b with radius r.
 *
 * p: Point to test in the SDF.
 * a: Origin point of the segment.
 * b: End point of the segment.
 * r: Radius of the line segment.
 */
float segmentSDF(vec3 p, vec3 a, vec3 b, float r) {
    float h = min(1.0, max(0.0, dot(p - a, b - a) / dot(b - a, b - a)));
    return length(p - a - (b - a) * h) - r;
}

/**
 * SDF for a sphere centered at the origin with radius 1.0;
 *
 * p: Point to test in the sphere
 * origin: Origin point of the sphere.
 * radius: Radius of the sphere.
 */
float sphereSDF(vec3 p, vec3 origin, float radius) {
    return distance(p, origin) - radius;
}

/**
 * SDF for a torus geometry.
 *
 * p: Point to test in the surface.
 * o: Center point of the torus.
 * r: Radius of the torus
 * t: Thickness of the torus arc.
 */
float torusSDF(vec3 p, vec3 o, float r, float t) {
    p -= o;
    vec2 q = vec2(length(p.xz) - r, p.y);
    return length(q) - t;
}

/**
 * SDF for a cube centered at the origin with width = height = length = 2.0
 *
 * p: Point to test in the cube
 * center: Center of the cube
 * size: Size of the cube
 */
float cubeSDF(vec3 p, vec3 center, vec3 size) {
    // Apply offset the point to move the cube
    p -= center;

    // If d.x < 0, then -1 < p.x < 1, and same logic applies to p.y, p.z, if all components of d are negative, then p is inside the unit cube
    vec3 d = abs(p) - size;

    // Assuming p is inside the cube, how far is it from the surface. Result will be negative or zero.
    float insideDistance = min(max(d.x, max(d.y, d.z)), 0.0);

    // Assuming p is outside the cube, how far is it from the surface. Result will be positive or zero.
    float outsideDistance = length(max(d, 0.0));

    return insideDistance + outsideDistance;
}

/**
 * Plane SDF.
 *
 * p: Point to test in the plane.
 * h: Height of the plane.
 */
float planeSDF(vec3 p, float h)
{
    return p.y - h;
}

/**
 * SDF for a hexagonal prism.
 *
 * p: Point to test in the prism.
 * h: Size of the prism.
 */
float hexPrismSDF(vec3 p, vec2 h)
{
    vec3 q = abs(p);

    const vec3 k = vec3(-0.8660254, 0.5, 0.57735);
    p = abs(p);
    p.xy -= 2.0*min(dot(k.xy, p.xy), 0.0)*k.xy;

    vec2 d = vec2(length(p.xy - vec2(clamp(p.x, -k.z*h.x, k.z*h.x), h.x))*sign(p.y - h.x), p.z-h.y);

    return min(max(d.x,d.y),0.0) + length(max(d,0.0));
}

/**
 * Iso surface of a SDF can be used to make the surface on an SDF rounder.
 *
 * For all point in the SDF an iso surface is a surface distanced from the original surface by r.
 *
 * a: Distance to the base surface
 * r: Radius of the iso surface.
 */
float isoSurfaceSDF(float a, float r) {
    return a - r;
}

/**
 * Smooth union between two SDF.
 *
 * https://www.iquilezles.org/www/articles/smin/smin.htm
 *
 * a: SDF result a to be joined.
 * b: SDF result b to be joined.
 * k: Constant of smoothness to be applied in the union.
 */
float smin(float a, float b, float k) {
    float h = max(k-abs(a-b), 0.0)/k;
    return min(a, b) - h*h*h*k*(1.0/6.0);
}

/**
 * Apply noise to a SDF using noise texture.
 *
 * d: Distance to apply noise to.
 * scale: Scale of the noise to be applied
 */
float noise(float a, float scale) {
    a += texture(noise_text, v_uv).r / (255.0 / scale);
    return a;
}

/**
 * SDF describing the scene.
 * Absolute value of the return value indicates the distance to the surface.
 * Sign indicates whether the point is inside or outside the surface,negative indicating inside.
 *
 * p: Point to test in the scene
 */
float sceneSDF(vec3 p) {
    float o = sphereSDF(p, vec3(-0.8, 0, 0), 0.5);
    o = unionSDF(o, cubeSDF(p, vec3(0, 2.0, 2.0), vec3(0.6, 0.3, 0.3)));
    o = smin(o, cubeSDF(p, vec3(0, 2.5, 2.3), vec3(0.3, 0.6, 0.3)), 0.4);
    o = unionSDF(o, segmentSDF(p, vec3(0, 0, 0), vec3(0.0, 2.0, 0), 0.3));
    o = unionSDF(o, torusSDF(p, vec3(0, 1.0, 0), 1.1, 0.3));

    o = unionSDF(o, planeSDF(p, -0.5));

    return o; // noise(o, 4.0);
}

/**
 * Return the shortest distance from the eyepoint to the scene surface along the marching direction.
 * If no part of the surface is found between start and end, return end.
 *
 * eye: the eye point, acting as the origin of the ray
 * marchingDirection: the normalized direction to march in
 * start: the starting distance away from the eye
 * end: the max distance away from the ey to march before giving up
 */
float shortestDistanceToSurface(vec3 eye, vec3 marchingDirection, float start, float end) {
    float depth = start;

    for (int i = 0; i < MAX_MARCHING_STEPS; i++) {
        float dist = sceneSDF(eye + depth * marchingDirection);
        if (dist < EPSILON) {
            return depth;
        }

        depth += dist;

        if (depth >= end) {
            return end;
        }
    }
    return end;
}

/**
 * Return the normalized direction to march in from the eye point for a single pixel.
 *
 * fieldOfView: vertical field of view in degrees
 * size: resolution of the output image
 * fragCoord: the x,y coordinate of the pixel in the output image
 */
vec3 rayDirection(float fieldOfView, vec2 size, vec2 fragCoord) {
    vec2 xy = fragCoord - size / 2.0;
    float z = size.y / tan(radians(fieldOfView) / 2.0);
    return normalize(vec3(xy, -z));
}

/**
 * Using the gradient of the SDF, estimate the normal on the surface at point p.
 */
vec3 estimateNormal(vec3 p) {
    return normalize(vec3(
        sceneSDF(vec3(p.x + EPSILON, p.y, p.z)) - sceneSDF(vec3(p.x - EPSILON, p.y, p.z)),
        sceneSDF(vec3(p.x, p.y + EPSILON, p.z)) - sceneSDF(vec3(p.x, p.y - EPSILON, p.z)),
        sceneSDF(vec3(p.x, p.y, p.z  + EPSILON)) - sceneSDF(vec3(p.x, p.y, p.z - EPSILON))
    ));
}

/**
 * Lighting contribution of a single point light source via Phong illumination.
 *
 * The vec3 returned is the RGB color of the light's contribution.
 *
 * k_a: Ambient color
 * k_d: Diffuse color
 * k_s: Specular color
 * alpha: Shininess coefficient
 * p: position of point being lit
 * eye: the position of the camera
 * lightPos: the position of the light
 * lightIntensity: color/intensity of the light
 */
vec3 phongContribForLight(vec3 k_d, vec3 k_s, float alpha, vec3 p, vec3 eye, vec3 lightPos, vec3 lightIntensity) {
    vec3 N = estimateNormal(p);
    vec3 L = normalize(lightPos - p);
    vec3 V = normalize(eye - p);
    vec3 R = normalize(reflect(-L, N));

    float dotLN = dot(L, N);
    float dotRV = dot(R, V);

    // Light not visible from this point on the surface
    if (dotLN < 0.0) {
        return vec3(0.0, 0.0, 0.0);
    }

    // Light reflection in opposite direction as viewer, apply only diffuse component
    if (dotRV < 0.0) {
        return lightIntensity * (k_d * dotLN);
    }

    return lightIntensity * (k_d * dotLN + k_s * pow(dotRV, alpha));
}

/**
 * Lighting via Phong illumination.
 *
 * The vec3 returned is the RGB color of that point after lighting is applied.

 * k_a: Ambient color
 * k_d: Diffuse color
 * k_s: Specular color
 * alpha: Shininess coefficient
 * p: position of point being lit
 * eye: the position of the camera
 */
vec3 phongIllumination(vec3 k_a, vec3 k_d, vec3 k_s, float alpha, vec3 p, vec3 eye) {
    // Ambient Light
    const vec3 ambientLight = 0.5 * vec3(1.0, 1.0, 1.0);
    vec3 color = ambientLight * k_a;

    // Point Light 1
    vec3 light1Pos = vec3(4.0 * sin(time), 2.0, 4.0 * cos(time));
    vec3 light1Intensity = vec3(0.4, 0.4, 0.4);

    color += phongContribForLight(k_d, k_s, alpha, p, eye, light1Pos, light1Intensity);

    // Point Light 2
    vec3 light2Pos = vec3(2.0 * sin(0.37 * time), 2.0 * cos(0.37 * time), 2.0);
    vec3 light2Intensity = vec3(0.4, 0.4, 0.4);

    color += phongContribForLight(k_d, k_s, alpha, p, eye, light2Pos, light2Intensity);
    return color;
}

/**
 * Return a transformation matrix that will transform a ray from view space to world coordinates, given the eye point, the camera target, and an up vector.
 *
 * This assumes that the center of the camera is aligned with the negative z axis in view space when calculating the ray marching direction.
 */
mat4 viewMatrix(vec3 eye, vec3 center, vec3 up) {
    vec3 f = normalize(center - eye);
    vec3 s = normalize(cross(f, up));
    vec3 u = cross(s, f);

    return mat4(
        vec4(s, 0.0),
        vec4(u, 0.0),
        vec4(-f, 0.0),
        vec4(0.0, 0.0, 0.0, 1)
    );
}

void main() {
    vec2 fragCoord = gl_FragCoord.xy;

    vec3 viewDir = rayDirection(60.0, resolution.xy, fragCoord);

    mat4 viewToWorld = viewMatrix(eye, vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
    vec3 worldDir = (viewToWorld * vec4(viewDir, 0.0)).xyz;

    float dist = shortestDistanceToSurface(eye, worldDir, MIN_DIST, MAX_DIST);

    // Didn't hit anything draw background
    if (dist > MAX_DIST - EPSILON) {
        // color = vec4(texture(noise_text, v_uv).rgb, 0.0);
        color = vec4(0, 0, 0, 1);
        return;
    }

    // The closest point on the surface to the eyepoint along the view ray
    vec3 p = eye + dist * worldDir;
    vec3 K_a = vec3(0.1, 0.1, 0.1);
    vec3 K_d = vec3(0.3, 0.2, 0.2);
    vec3 K_s = vec3(1.0, 1.0, 1.0);
    float shininess = 10.0;

    vec3 phongColor = phongIllumination(K_a, K_d, K_s, shininess, p, eye);

    color = vec4(phongColor, 1.0);
}