extern crate gl_helpers;

fn main() {
    println!("OpenGL Version: {}.{}", gl_helpers::gl_major(), gl_helpers::gl_minor());
    println!("GLSL Version: {}.{}", gl_helpers::glsl_major(), gl_helpers::glsl_minor());
}
