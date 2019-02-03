## Ray

A toy ray tracer engine as a practice to introductory computer graphics.

![](https://raw.githubusercontent.com/shouya/ray/master/gallery/triangle.png)

Featuring Blender-like composable shader and more.

Checkout [gallery](https://github.com/shouya/ray/tree/master/gallery) for more!

## Features

- Basic shaders
  - Phong
  - Diffuse
  - Reflection
  - Refraction
  - (WIP) Chessboard

- Composite shaders and preset materials
  - Transparent (Fresnel)
  - Solid
  - Glass
  - Mirror

- Light sources
  - Point light

- Objects
  - Sphere
  - Triangle & Rectangle (one-sided or double-sided)
  - Triangle mesh (imported via .obj file), vertex normals supported
  - Chessboard plane
  - Object transformations (rotation/translation/scaling)

- Camera & Scene
  - Perspective & Orthogonal view
  - Ambient light and ambient color

## Planned features

- Rough surface shader
- Skybox
- Bezier surface
- Area light & soft shadow
- Illuminating object
- Monte-Carlo tracer
- Scripting language for describing the scene

## References

- [Introduction to Ray Tracing: a Simple Method for Creating 3D Images](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-ray-tracing/how-does-it-work)
- [Reflections and Refractions in Ray Tracing](https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf)
- [Reflection, Refraction and Fresnel](http://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel)
- [Ambient, Diffuse, Specular and Emissive lighting](https://bassemtodary.wordpress.com/2013/04/13/ambient-diffuse-specular-and-emissive-lighting/)
- [StackOverflow: Refraction in Raytracing?](https://stackoverflow.com/questions/26087106/refraction-in-raytracing)
- [Cornell CS4620/5620: Lecture 35](http://www.cs.cornell.edu/courses/cs4620/2012fa/lectures/35raytracing.pdf)
- [Bézier Curves and Surfaces: the Utah Teapot](https://www.scratchapixel.com/lessons/advanced-rendering/bezier-curve-rendering-utah-teapot/bezier-surface)
- [Learn Computer Graphics From Scratch!](https://www.scratchapixel.com/index.php?redirect)
- [Countless articles on scratchapixel](https://www.scratchapixel.com)
- [Transformation Hierarchy](http://groups.csail.mit.edu/graphics/classes/6.837/F03/lectures/05_transformation_hierarchy.ppt)
- [A brief computer graphics / rendering course](https://github.com/ssloy/tinyraytracer)
- Akenine-Moller et al, "Real-Time Rendering", 3rd edition