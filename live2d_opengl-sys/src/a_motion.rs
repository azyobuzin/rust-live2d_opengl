use super::*;

// extern { }

pub struct AMotion {
    ptr: LDObjectPtr,
}

impl Drop for AMotion {
    fn drop(&mut self) {
        unsafe { super::delete_ld_object(self.ptr) }
    }
}

impl LDObject for AMotion {
    fn get_ptr(&self) -> LDObjectPtr {
        self.ptr
    }
}

impl AMotion {
    pub unsafe fn from_ptr(ptr: LDObjectPtr) -> AMotion {
        AMotion { ptr: ptr }
    }
}
