use std::mem;
use sys;
use sys::LDObject;

#[derive(Debug)]
pub struct Live2DMotion<'a> {
    #[allow(dead_code)]
    global: &'a super::Live2D,
    inner: sys::Live2DMotion,
}

impl<'a> Live2DMotion<'a> {
    pub fn new(global: &'a super::Live2D, motion: sys::Live2DMotion) -> Live2DMotion<'a> {
        Live2DMotion {
            global: global,
            inner: motion,
        }
    }

    pub fn load(global: &'a super::Live2D, data: &[u8]) -> Live2DMotion<'a> {
        // 失敗したかどうか判断できない……
        Live2DMotion::new(global, sys::Live2DMotion::loadMotion(data))
    }

    pub fn set_loop(&mut self, b: bool) {
        self.inner.setLoop(b)
    }

    pub fn is_loop(&self) -> bool {
        self.inner.isLoop()
    }
}

impl<'a> AsRef<sys::Live2DMotion> for Live2DMotion<'a> {
    fn as_ref(&self) -> &sys::Live2DMotion {
        &self.inner
    }
}

impl <'a> AsMut<sys::Live2DMotion> for Live2DMotion<'a> {
    fn as_mut(&mut self) -> &mut sys::Live2DMotion {
        &mut self.inner
    }
}

#[derive(Debug)]
pub struct MotionQueueManager<'a> {
    #[allow(dead_code)]
    global: &'a super::Live2D,
    inner: sys::MotionQueueManager,
}

impl<'a> MotionQueueManager<'a> {
    pub fn new(global: &'a super::Live2D) -> MotionQueueManager<'a> {
        MotionQueueManager {
            global: global,
            inner: sys::MotionQueueManager::new(),
        }
    }

    // 戻り値不明
    pub fn start_motion<'b, M>(&'b mut self, motion: M)
        where M: Into<Motion<'a, 'b>>
    {
        match motion.into() {
            Motion::Borrowed(x) => {
                self.inner.startMotion(x.inner.get_ptr(), false);
            }
            Motion::Owned(x) => {
                let p = x.inner.get_ptr();
                mem::forget(x); // Live2D 側に破棄させる
                self.inner.startMotion(p, true);
            }
        }
    }

    pub fn update_model(&mut self, model: &mut super::Live2DModel) -> bool {
        self.inner.updateParam(model.as_mut())
    }

    pub fn is_finished(&self) -> bool {
        self.inner.isFinished()
    }

    pub fn stop_all_motions(&mut self) {
        self.inner.stopAllMotions()
    }
}

impl<'a> AsRef<sys::MotionQueueManager> for MotionQueueManager<'a> {
    fn as_ref(&self) -> &sys::MotionQueueManager {
        &self.inner
    }
}

impl <'a> AsMut<sys::MotionQueueManager> for MotionQueueManager<'a> {
    fn as_mut(&mut self) -> &mut sys::MotionQueueManager {
        &mut self.inner
    }
}

#[derive(Debug)]
pub enum Motion<'a: 'b, 'b> {
    Borrowed(&'b Live2DMotion<'a>),
    Owned(Live2DMotion<'a>),
}

impl<'a, 'b> From<&'b Live2DMotion<'a>> for Motion<'a, 'b> {
    fn from(x: &'b Live2DMotion<'a>) -> Motion<'a, 'b> {
        Motion::Borrowed(x)
    }
}

impl<'a, 'b> From<Live2DMotion<'a>> for Motion<'a, 'b> {
    fn from(x: Live2DMotion<'a>) -> Motion<'a, 'b> {
        Motion::Owned(x)
    }
}
