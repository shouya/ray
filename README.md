## Ray

A toy ray tracer engine (and probably) as a practice of computer graphics.

![](https://raw.githubusercontent.com/shouya/ray/master/gallery/triangle.png)

Checkout [gallery](https://github.com/shouya/ray/tree/master/gallery) for more!

## Roadmap

- Basic types
  - [x] V3
  - [x] Color
  - [x] Trig/Plane
  - [x] Ray
  - [x] Hit
  - [x] Bounding box
- Scene
  - [x] Screen-Camera coordinates translation
  - [x] Ray generation for a screen pixel
  - [x] Support both orthogonal and perspective view
  - [x] Multiple light sources
- Objects
  - [x] Sphere
  - [x] Chessboard background
  - [x] Triangle
  - [x] Rectangle
  - [x] Triangular mesh
  - [ ] Polygon-faced mesh (optional)
  - [ ] Bezier mesh
- Lights
  - [x] AmbientLight
  - [x] PointLight
  - [ ] SpotLight
- Material support
  - [x] Solid diffusive object (Material::Solid)
  - [x] Reflective object (Material::Mirror)
  - [x] Transparent object (Material::Glass)
  - [x] Rough reflective object (Material::FrostedMirror)
  - [x] Rough transparent object (Material::FrostedGlass)
  - [ ] Metal-like objeect (Material::Metal)
- Ray tracer
  - [x] Shadows
  - [x] Render diffusive surface
  - [x] Render specular reflection
  - [x] General reflection (e.g. mirror)
  - [x] Rough surface reflection (e.g. frosted mirror)
  - [x] Support refraction
  - [x] Fresnel effect
  - [x] Rough surface refraction (e.g. frosted glass)
  - [ ] Correct shadow for transparent objects
  - [ ] Texture mapping
- [ ] Rasterization
- Intersection detection
  - [x] Sphere
  - [x] Triangle
  - [x] Plane
  - [x] Mesh (Trig/Poly)
  - [x] Bounding box
  - [x] Subdividing clusters for mesh
  - [x] Möller–Trumbore algorithm for triangles
- Misc
  - [x] Load triangular mesh from .obj model
  - [ ] Load poly-faced mesh from .obj model
  - [ ] Load bezier mesh from .obj model
  - [ ] Animation generation
  - [ ] Load scene from a DSL (or [dyon](https://github.com/PistonDevelopers/dyon)?)


## References

- [Introduction to Ray Tracing: a Simple Method for Creating 3D Images](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-ray-tracing/how-does-it-work)
- [Reflections and Refractions in Ray Tracing](https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf)
- [Reflection, Refraction and Fresnel](http://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel)
- [Ambient, Diffuse, Specular and Emissive lighting](https://bassemtodary.wordpress.com/2013/04/13/ambient-diffuse-specular-and-emissive-lighting/)
- [StackOverflow: Refraction in Raytracing?](https://stackoverflow.com/questions/26087106/refraction-in-raytracing)
- [Cornell CS4620/5620: Lecture 35](http://www.cs.cornell.edu/courses/cs4620/2012fa/lectures/35raytracing.pdf)
- [Bézier Curves and Surfaces: the Utah Teapot](https://www.scratchapixel.com/lessons/advanced-rendering/bezier-curve-rendering-utah-teapot/bezier-surface)
- [Learn Computer Graphics From Scratch!](https://www.scratchapixel.com/index.php?redirect)
