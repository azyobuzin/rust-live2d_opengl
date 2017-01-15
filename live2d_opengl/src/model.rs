use super::gl;

use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::{c_float, c_int, c_uint};
use glium;
use sys;

pub struct Live2DModel<'a, 'b> {
    #[allow(dead_code)]
    global: &'a super::Live2D,
    inner: sys::Live2DModelWinGL,
    textures: HashMap<c_int, Box<ModelTexture<'b>>>,
}

impl<'a, 'b> Live2DModel<'a, 'b> {
    pub fn new(global: &'a super::Live2D, model: sys::Live2DModelWinGL) -> Live2DModel<'a, 'b> {
        Live2DModel {
            global: global,
            inner: model,
            textures: HashMap::new(),
        }
    }

    pub fn load(global: &'a super::Live2D, data: &[u8]) -> Result<Live2DModel<'a, 'b>, ()> {
        let model = sys::Live2DModelWinGL::loadModel(data)?;
        if model.getCanvasWidth() == 0.0 || model.getCanvasHeight() == 0.0 {
            Err(())
        } else {
            Ok(Live2DModel::new(global, model))
        }
    }

    pub fn get_param(&self, param_id: &str) -> f32 {
        self.inner.getParamFloat(unsafe { &CString::from_vec_unchecked(param_id.into()) })
    }

    pub fn set_param(&mut self, param_id: &str, value: f32) {
        self.inner.setParamFloat(
            unsafe { &CString::from_vec_unchecked(param_id.into()) },
            value,
            1.0
        )
    }

    pub fn set_param_with_weight(&mut self, param_id: &str, value: f32, weight: f32) {
        self.inner.setParamFloat(
            unsafe { &CString::from_vec_unchecked(param_id.into()) },
            value,
            weight
        )
    }

    pub fn update(&mut self) {
        self.inner.update();
    }

    pub fn draw(&self, context: &glium::backend::Context, draw_parameters: &super::Live2DDrawParameters) {
        unsafe {
            context.exec_in_context(move || {
                gl::glPushAttrib(gl::GL_ALL_ATTRIB_BITS);

                if let Some(viewport) = draw_parameters.viewport {
                    gl::glViewport(viewport.left as gl::GLint, viewport.bottom as gl::GLint,
                        viewport.width as gl::GLsizei, viewport.height as gl::GLsizei);
                } else {
                    let (w, h) = context.get_framebuffer_dimensions();
                    gl::glViewport(0, 0, w as gl::GLsizei, h as gl::GLsizei);
                }

                if let Some(scissor) = draw_parameters.scissor {
                    gl::glScissor(scissor.left as gl::GLint, scissor.bottom as gl::GLint,
                        scissor.width as gl::GLsizei, scissor.height as gl::GLsizei);
                    gl::glEnable(gl::GL_SCISSOR_TEST);
                } else {
                    gl::glDisable(gl::GL_SCISSOR_TEST);
                }

                self.inner.draw();

                gl::glPopAttrib();
            })
        }
    }

    pub fn width(&self) -> f32 {
        self.inner.getCanvasWidth()
    }

    pub fn height(&self) -> f32 {
        self.inner.getCanvasHeight()
    }

    pub fn set_premultiplied_alpha(&mut self, b: bool) {
        self.inner.setPremultipliedAlpha(b)
    }

    pub fn is_premultiplied_alpha(&self) -> bool {
        self.inner.isPremultipliedAlpha()
    }

    pub fn set_texture(&mut self, texture_no: i32, texture: Box<ModelTexture<'b>>) {
        unsafe { self.inner.setTexture(texture_no, texture.get_id()) }
        self.textures.insert(texture_no, texture);
    }

    pub fn set_matrix(&mut self, matrix: &[f32; 16]) {
        unsafe { self.inner.setMatrix(matrix.as_ptr() as *mut c_float) }
    }
}

impl<'a, 'b> AsRef<sys::Live2DModelWinGL> for Live2DModel<'a, 'b> {
    fn as_ref(&self) -> &sys::Live2DModelWinGL {
        &self.inner
    }
}

impl <'a, 'b> AsMut<sys::Live2DModelWinGL> for Live2DModel<'a, 'b> {
    fn as_mut(&mut self) -> &mut sys::Live2DModelWinGL {
        &mut self.inner
    }
}

pub trait ModelTexture<'a> {
    fn get_id(&self) -> c_uint;
}

macro_rules! impl_model_texture {
    ($t:ty) => (
        impl<'a> ModelTexture<'a> for $t {
            fn get_id(&self) -> c_uint {
                glium::GlObject::get_id(self)
            }
        }

        impl<'a> ModelTexture<'a> for &'a $t {
            fn get_id(&self) -> c_uint {
                glium::GlObject::get_id(*self)
            }
        }
    )
}

impl_model_texture!(glium::texture::Texture2d);
impl_model_texture!(glium::texture::CompressedTexture2d);
impl_model_texture!(glium::texture::SrgbTexture2d);
impl_model_texture!(glium::texture::CompressedSrgbTexture2d);
