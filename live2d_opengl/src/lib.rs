extern crate live2d_opengl_sys as sys;
extern crate glium;
#[macro_use] extern crate lazy_static;

#[cfg(feature = "model-json")]
extern crate image;
#[cfg(feature = "model-json")]
extern crate rustc_serialize;

use std::sync;

mod global_state;
pub use global_state::UserTime;

pub mod model;
pub use model::Live2DModel;

pub mod motion;
pub use motion::{Live2DMotion, MotionQueueManager};

mod gl;

#[derive(Debug, Clone)]
pub struct Live2D {
    inner: sync::Arc<global_state::GlobalState>,
}

impl Live2D {
    pub fn get() -> Live2D {
        Live2D { inner: global_state::get() }
    }

    pub fn user_time(&self) -> &sync::RwLock<UserTime> {
        &self.inner.user_time
    }

    pub fn load_model<'a, 'b>(&'a self, data: &[u8]) -> Result<Live2DModel<'a, 'b>, ()> {
        Live2DModel::load(self, data)
    }

    // TODO: いつかやる
    /*
    #[cfg(feature = "model-json")]
    pub fn load_model_json<'a, 'b, P>(&'a self, path: P) -> Result<Live2DModel<'a, 'b>, ()> {
        // TODO: エラー型
        unimplemented!()
    }
    */

    pub fn load_motion<'a>(&'a self, data: &[u8]) -> Live2DMotion {
        Live2DMotion::load(self, data)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Live2DDrawParameters {
    pub viewport: Option<glium::Rect>,
    pub scissor: Option<glium::Rect>,
}
