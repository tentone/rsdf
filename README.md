# Rust SDF
- Small SDF (Signed Distance Field) rendering experiments using Rust and OpenGL.
- With support for desktop OpenGL and WebGL trough web assembly (check [geotoy](https://github.com/fitzgen/geotoy/tree/master/src) for an example using WASM and glium)
- [glium](https://docs.rs/glium/0.29.0/glium/) and [glutin](https://docs.rs/glutin/0.26.0/glutin/) are used to access OpenGL and obtain access to a window and input.



### References

Some references for more information about SDF modelling and rendering.

- [Inigo Quilez SDF Primitives list](https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm)
- [Smooth blending between SDF](https://www.iquilezles.org/www/articles/smin/smin.htm)
- [Soft shadows in raymarched SDFs](https://www.iquilezles.org/www/articles/rmshadows/rmshadows.htm) 
- [Ray Marching and Signed Distance Functions](http://jamie-wong.com/2016/07/15/ray-marching-signed-distance-functions/)
- [Signed Distance Fields Using Single-Pass GPU Scan Conversion of Tetrahedra](https://developer.nvidia.com/gpugems/gpugems3/part-v-physics-simulation/chapter-34-signed-distance-fields-using-single-pass-gpu)