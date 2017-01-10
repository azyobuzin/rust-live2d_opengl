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
    pub fn getParamFloat(&self, paramID: &CStr) -> c_float {
        unsafe { ALive2DModel_getParamFloat(self.ptr, paramID.as_ptr()) }
    }

    pub fn setParamFloatWithWeight(&mut self, paramID: &CStr, value: c_float, weight: c_float) {
        unsafe { ALive2DModel_setParamFloat(self.ptr, paramID.as_ptr(), value, weight) }
    }

    pub fn setParamFloat(&mut self, paramID: &CStr, value: c_float) {
        self.setParamFloatWithWeight(paramID, value, 1.0)
    }

    pub fn update(&mut self) {
        unsafe { ALive2DModel_update(self.ptr) }
    }

    pub fn draw(&mut self) {
        unsafe { ALive2DModel_draw(self.ptr) }
    }

    pub fn getCanvasWidth(&self) -> c_float {
        unsafe { ALive2DModel_getCanvasWidth(self.ptr) }
    }

    pub fn getCanvasHeight(&self) -> c_float {
        unsafe { ALive2DModel_getCanvasHeight(self.ptr) }
    }

    pub fn setPremultipliedAlpha(&mut self, b: bool) {
        let x = if b { 1 } else { 0 };
        unsafe { ALive2DModel_setPremultipliedAlpha(self.ptr, x) }
    }

    pub fn isPremultipliedAlpha(&self) -> bool {
        unsafe { ALive2DModel_isPremultipliedAlpha(self.ptr) != 0 }
    }
}
