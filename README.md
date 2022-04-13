<h1 align="center">Cosmic</h1>
<h4 align="center">Easy to use, configurable C/C++ package manager and build system</h4>
<br>
<b>Cosmic is still in early alpha, expect bugs and breakage</b> (but please make an issue!)
<br>
<br>

Visit https://docs.cosmic.rs for extended documentation

## Example Cosmic Definition:

```
package bin example_gl;

meta {
    authors: [ "Nebula<nebula@cosmic.rs>" ],
    description: "Example Cosmic Definition for an OpenGL project",
    license: "MIT"
    cxx: true
}

define {
    GLFW_BUILD_DOCS: false
}

compilers {
    c: "clang",
    cxx: "clang++"
}

depends(include, static) {
    glfw,
    glm,
    glad
}
```
