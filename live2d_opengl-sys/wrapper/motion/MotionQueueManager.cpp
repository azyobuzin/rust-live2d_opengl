#include "../w_common.h"
#include "motion/MotionQueueManager.h"

using namespace live2d;

extern "C" {
    MotionQueueManager* MotionQueueManager_new() {
        return new MotionQueueManager;
    }

    int MotionQueueManager_startMotion(MotionQueueManager *p, AMotion *motion, w_bool autoDelete) {
        return p->startMotion(motion, autoDelete);
    }

    w_bool MotionQueueManager_updateParam(MotionQueueManager *p, ALive2DModel *model) {
        return p->updateParam(model);
    }

    w_bool MotionQueueManager_isFinished(MotionQueueManager *p) {
        return p->isFinished();
    }

    void MotionQueueManager_stopAllMotions(MotionQueueManager *p) {
        p->stopAllMotions();
    }

    void MotionQueueManager_setMotionDebugMode(MotionQueueManager *p, w_bool f) {
        p->setMotionDebugMode(f);
    }

    MotionQueueEnt* MotionQueueManager_getMotionQueueEnt(MotionQueueManager *p, int entNo) {
        return p->getMotionQueueEnt(entNo);
    }
}
