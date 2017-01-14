use super::*;
use super::w_bool;

use std::os::raw::c_int;

extern {
    fn MotionQueueManager_new() -> LDObjectPtr;
    fn MotionQueueManager_startMotion(p: LDObjectPtr, motion: LDObjectPtr, autoDelete: w_bool) -> c_int;
    fn MotionQueueManager_updateParam(p: LDObjectPtr, model: LDObjectPtr) -> w_bool;
    fn MotionQueueManager_isFinished(p: LDObjectPtr) -> w_bool;
    fn MotionQueueManager_stopAllMotions(p: LDObjectPtr);
}

pub struct MotionQueueManager {
    ptr: LDObjectPtr,
}

impl Drop for MotionQueueManager {
    fn drop(&mut self) {
        unsafe { super::delete_ld_object(self.ptr) }
    }
}

impl LDObject for MotionQueueManager {
    fn get_ptr(&self) -> LDObjectPtr {
        self.ptr
    }
}

impl MotionQueueManager {
    pub unsafe fn from_ptr(ptr: LDObjectPtr) -> MotionQueueManager {
        MotionQueueManager { ptr: ptr }
    }

    pub fn new() -> MotionQueueManager {
        unsafe { MotionQueueManager::from_ptr(MotionQueueManager_new()) }
    }

    pub fn startMotion(&mut self, motion: &AMotion, autoDelete: bool) -> c_int {
        let auto_delete = if autoDelete { 1 } else { 0 };
        unsafe { MotionQueueManager_startMotion(self.ptr, motion.get_ptr(), auto_delete) }
    }

    pub fn updateParam(&mut self, model: &mut ALive2DModel) -> bool {
        unsafe { MotionQueueManager_updateParam(self.ptr, model.get_ptr()) != 0 }
    }

    pub fn isFinished(&self) -> bool {
        unsafe { MotionQueueManager_isFinished(self.ptr) != 0 }
    }

    pub fn stopAllMotions(&mut self) {
        unsafe { MotionQueueManager_stopAllMotions(self.ptr) }
    }
}
