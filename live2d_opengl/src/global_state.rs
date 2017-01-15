use std;
use std::sync;
use sys;

#[derive(Debug)]
pub struct GlobalState {
    pub user_time: sync::RwLock<UserTime>,
}

impl GlobalState {
    fn new() -> GlobalState {
        unsafe { sys::init(); }

        GlobalState {
            user_time: sync::RwLock::new(UserTime(Default::default())),
        }
    }
}

impl Drop for GlobalState {
    fn drop(&mut self) {
        // super::Live2D がすべて破棄されたら dispose
        unsafe { sys::dispose(); }
    }
}

lazy_static! {
    static ref GLOBAL_STATE: sync::RwLock<sync::Weak<GlobalState>> = Default::default();
}

pub fn get() -> sync::Arc<GlobalState> {
    let global_state: &sync::RwLock<sync::Weak<GlobalState>> = &GLOBAL_STATE;

    loop {
        if let Some(x) = global_state.read().unwrap().upgrade() {
            return x;
        }

        // アップグレードできないので新規作成
        match global_state.try_write() {
            Ok(mut w) => {
                let result = sync::Arc::new(GlobalState::new());
                *w = sync::Arc::downgrade(&result);
                return result;
            }
            Err(sync::TryLockError::WouldBlock) => (), // 別スレッドが作成中なのでリトライ
            Err(sync::TryLockError::Poisoned(x)) => panic!(format!("{}", x)),
        }
    }
}

// 外部から作れないように PhantomData を仕込んでおく
#[derive(Debug)]
pub struct UserTime(std::marker::PhantomData<()>);

impl UserTime {
    pub fn get_in_msec(&self) -> i64 {
        sys::ut_system::getUserTimeMSec()
    }

    pub fn set_in_msec(&mut self, t: i64) {
        unsafe { sys::ut_system::setUserTimeMSec(t) }
    }

    pub fn set_system_time(&mut self) -> i64 {
        unsafe { sys::ut_system::updateUserTimeMSec() }
    }

    pub fn reset(&mut self) {
        unsafe { sys::ut_system::resetUserTimeMSec() }
    }

    pub fn get_system_time_in_msec() -> i64 {
        sys::ut_system::getTimeMSec()
    }
}
