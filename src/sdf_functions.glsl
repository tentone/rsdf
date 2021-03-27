
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
 * r: Radius of the torus
 * t: Thickness of the torus arc.
 */
float torusSDF(vec3 p, float r, float t) {
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
 * https://www.iquilezles.org/www/articles/smin/smin.htm
 */
float smin(float a, float b, float k)
{
    float h = max( k-abs(a-b), 0.0 )/k;
    return min( a, b ) - h*h*h*k*(1.0/6.0);
}
