use std::os::raw::{c_int, c_uint};

pub type GLenum = c_uint;
pub type GLbitfield = c_uint;
pub type GLint = c_int;
pub type GLsizei = c_int;

pub const GL_ALL_ATTRIB_BITS: GLbitfield = 0x000fffff;
pub const GL_SCISSOR_TEST: GLenum = 0x0C11;

// libl2dwrapper が opengl32 とリンクしているのを利用する
extern "system" {
    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    pub fn glPushAttrib(mask: GLbitfield);
    pub fn glPopAttrib();
    pub fn glEnable(cap: GLenum);
    pub fn glDisable(cap: GLenum);
}
