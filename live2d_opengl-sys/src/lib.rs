#![allow(non_snake_case, non_camel_case_types)]

use std::os::raw::{c_float, c_int, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulonglong, c_ushort, c_void};

// bool representaion in the wrapper
type w_bool = c_int;

pub type l2d_uvmapf = c_float;
pub type l2d_pointf = c_float;
pub type l2d_paramf = c_float;
pub type l2d_index = c_ushort;
pub type l2d_order = c_short;
pub type l2d_float = c_float;
pub type l2d_int8 = c_schar;
pub type l2d_uint8 = c_uchar;
pub type l2d_int16 = c_short;
pub type l2d_uint16 = c_ushort;
pub type l2d_int32 = c_int;
pub type l2d_uint32 = c_uint;
pub type l2d_int64 = c_longlong;
pub type l2d_uint64 = c_ulonglong;

pub type LDObjectPtr = *mut c_void;

extern {
    fn delete_ld_object(obj: LDObjectPtr);
}

pub trait LDObject {
    fn get_ptr(&self) -> LDObjectPtr;
}

mod live2d;
pub use self::live2d::*;

mod a_live2d_model;
pub use self::a_live2d_model::*;

mod live2d_model_wingl;
pub use self::live2d_model_wingl::*;

mod a_motion;
pub use self::a_motion::*;

mod live2d_motion;
pub use self::live2d_motion::*;

mod motion_queue_manager;
pub use self::motion_queue_manager::*;

pub mod ut_system;
