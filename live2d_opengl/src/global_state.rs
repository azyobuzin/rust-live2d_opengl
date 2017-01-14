use std::sync;
use sys;

pub struct GlobalState;

impl GlobalState {
    fn new() -> GlobalState {
        unsafe { sys::init(); }
        GlobalState
    }
}

impl Drop for GlobalState {
    fn drop(&mut self) {
        unsafe { sys::dispose(); }
    }
}

lazy_static! {
    static ref GLOBAL_STATE: sync::RwLock<sync::Weak<GlobalState>> = Default::default();
}

pub fn get() -> sync::Arc<GlobalState> {
    if let Some(x) = GLOBAL_STATE.read().unwrap().upgrade() {
        return x;
    }

    // アップグレードできないので新規作成
    match GLOBAL_STATE.try_write() {
        Ok(mut w) => {
            let result = sync::Arc::new(GlobalState::new());
            *w = sync::Arc::downgrade(&result);
            result
        }
        Err(sync::TryLockError::WouldBlock) => {
            // 別スレッドが作成中なのでリトライ
            get()
        }
        Err(sync::TryLockError::Poisoned(x)) => panic!(format!("{}", x))
    }
}
