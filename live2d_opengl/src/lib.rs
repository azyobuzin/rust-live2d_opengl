extern crate live2d_opengl_sys as sys;
extern crate glium;
#[macro_use] extern crate lazy_static;

use std::sync;

mod global_state;

pub struct Live2D {
    inner: sync::Arc<global_state::GlobalState>,
}

impl Live2D {
    pub fn get() -> Live2D {
        Live2D { inner: global_state::get() }
    }
}
