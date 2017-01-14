use super::*;

use std;
use std::os::raw::{c_float, c_int};

extern {
    fn Live2DModelWinGL_loadModel(buf: *const c_void, bufSize: c_int) -> LDObjectPtr;
    fn Live2DModelWinGL_setTexture(p: LDObjectPtr, textureNo: c_int, openGLTextureNo: c_uint);
    fn Live2DModelWinGL_setMatrix(p: LDObjectPtr, matrix: *mut c_float);
}

pub struct Live2DModelWinGL {
    ptr: LDObjectPtr,
}

impl Drop for Live2DModelWinGL {
    fn drop(&mut self) {
        unsafe { super::delete_ld_object(self.ptr) }
    }
}

impl LDObject for Live2DModelWinGL {
    fn get_ptr(&self) -> LDObjectPtr {
        self.ptr
    }
}

impl Live2DModelWinGL {
    pub unsafe fn from_ptr(ptr: LDObjectPtr) -> Live2DModelWinGL {
        Live2DModelWinGL { ptr: ptr }
    }

    pub fn loadModel(buf: &[u8]) -> Result<Live2DModelWinGL, ()> {
        let ptr = unsafe {
            Live2DModelWinGL_loadModel(
                buf.as_ptr() as *const c_void,
                buf.len() as c_int
            )
        };

        if ptr.is_null() {
            Err(())
        } else {
            Ok(unsafe { Live2DModelWinGL::from_ptr(ptr) })
        }
    }

    pub unsafe fn setTexture(&mut self, textureNo: c_int, openGLTextureNo: c_uint) {
        Live2DModelWinGL_setTexture(self.ptr, textureNo, openGLTextureNo)
    }

    pub unsafe fn setMatrix(&mut self, matrix: *mut c_float) {
        Live2DModelWinGL_setMatrix(self.ptr, matrix)
    }
}

impl std::ops::Deref for Live2DModelWinGL {
    type Target = ALive2DModel;

    fn deref(&self) -> &ALive2DModel {
        unsafe { std::mem::transmute(self) }
    }
}

impl std::ops::DerefMut for Live2DModelWinGL {
    fn deref_mut(&mut self) -> &mut ALive2DModel {
        unsafe { std::mem::transmute(self) }
    }
}
