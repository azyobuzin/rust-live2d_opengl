use super::*;
use super::w_bool;

use std::ffi::CStr;
use std::os::raw::{c_char, c_float};

extern {
    fn ALive2DModel_getParamFloat(p: LDObjectPtr, paramID: *const c_char) -> c_float;
    fn ALive2DModel_setParamFloat(p: LDObjectPtr, paramID: *const c_char, value: c_float, weight: c_float);
    fn ALive2DModel_update(p: LDObjectPtr);
    fn ALive2DModel_draw(p: LDObjectPtr);
    fn ALive2DModel_getCanvasWidth(p: LDObjectPtr) -> c_float;
    fn ALive2DModel_getCanvasHeight(p: LDObjectPtr) -> c_float;
    fn ALive2DModel_setPremultipliedAlpha(p: LDObjectPtr, b: w_bool);
    fn ALive2DModel_isPremultipliedAlpha(p: LDObjectPtr) -> w_bool;
}

#[derive(Debug)]
pub struct ALive2DModel {
    ptr: LDObjectPtr,
}

impl Drop for ALive2DModel {
    fn drop(&mut self) {
        unsafe { super::delete_ld_object(self.ptr) }
    }
}

impl LDObject for ALive2DModel {
    fn get_ptr(&self) -> LDObjectPtr {
        self.ptr
    }
}

impl ALive2DModel {
    pub unsafe fn from_ptr(ptr: LDObjectPtr) -> ALive2DModel {
        ALive2DModel { ptr: ptr }
    }

    pub unsafe fn getParamFloat(&self, paramID: &CStr) -> c_float {
        ALive2DModel_getParamFloat(self.ptr, paramID.as_ptr())
    }

    pub unsafe fn setParamFloat(&mut self, paramID: &CStr, value: c_float, weight: c_float) {
        ALive2DModel_setParamFloat(self.ptr, paramID.as_ptr(), value, weight)
    }

    pub unsafe fn update(&mut self) {
        ALive2DModel_update(self.ptr)
    }

    pub unsafe fn draw(&self) {
        ALive2DModel_draw(self.ptr)
    }

    pub unsafe fn getCanvasWidth(&self) -> c_float {
        ALive2DModel_getCanvasWidth(self.ptr)
    }

    pub unsafe fn getCanvasHeight(&self) -> c_float {
        ALive2DModel_getCanvasHeight(self.ptr)
    }

    pub unsafe fn setPremultipliedAlpha(&mut self, b: bool) {
        ALive2DModel_setPremultipliedAlpha(self.ptr, if b { 1 } else { 0 })
    }

    pub unsafe fn isPremultipliedAlpha(&self) -> bool {
        ALive2DModel_isPremultipliedAlpha(self.ptr) != 0
    }
}
