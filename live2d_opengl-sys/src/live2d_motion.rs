use super::*;
use super::w_bool;

use std::os::raw::{c_int, c_void};

extern {
    fn Live2DMotion_loadMotion(buf: *const c_void, bufSize: c_int) -> LDObjectPtr;
    fn Live2DMotion_setLoop(p: LDObjectPtr, _loop: w_bool);
    fn Live2DMotion_isLoop(p: LDObjectPtr) -> w_bool;
}

pub struct Live2DMotion {
    ptr: LDObjectPtr,
}

impl Drop for Live2DMotion {
    fn drop(&mut self) {
        unsafe { super::delete_ld_object(self.ptr) }
    }
}

impl LDObject for Live2DMotion {
    fn get_ptr(&self) -> LDObjectPtr {
        self.ptr
    }
}

impl Live2DMotion {
    pub unsafe fn from_ptr(ptr: LDObjectPtr) -> Live2DMotion {
        Live2DMotion { ptr: ptr }
    }

    pub fn loadMotion(buf: &[u8]) -> Live2DMotion {
        unsafe {
            Live2DMotion::from_ptr(Live2DMotion_loadMotion(
                buf.as_ptr() as *const c_void,
                buf.len() as c_int
            ))
        }
    }

    pub fn setLoop(&mut self, _loop: bool) {
        let x = if _loop { 1 } else { 0 };
        unsafe { Live2DMotion_setLoop(self.ptr, x) }
    }

    pub fn isLoop(&self) -> bool {
        unsafe { Live2DMotion_isLoop(self.ptr) != 0 }
    }
}

impl std::ops::Deref for Live2DMotion {
    type Target = AMotion;

    fn deref(&self) -> &AMotion {
        unsafe { std::mem::transmute(self) }
    }
}

impl std::ops::DerefMut for Live2DMotion {
    fn deref_mut(&mut self) -> &mut AMotion {
        unsafe { std::mem::transmute(self) }
    }
}
