#include "../w_common.h"
#include "motion/Live2DMotion.h"

using namespace live2d;

extern "C" {
    void Live2DMotion_updateParamExe(Live2DMotion *p, ALive2DModel *model, long long timeMSec, float weight, MotionQueueEnt *motionQueueEnt) {
        p->updateParamExe(model, timeMSec, weight, motionQueueEnt);
    }

    void Live2DMotion_setLoop(Live2DMotion *p, w_bool _loop) {
        p->setLoop(_loop);
    }

    w_bool Live2DMotion_isLoop(Live2DMotion *p) {
        return p->isLoop();
    }

    void Live2DMotion_setLoopFadeIn(Live2DMotion *p, w_bool _loopFadeIn) {
        p->setLoopFadeIn(_loopFadeIn);
    }

    w_bool Live2DMotion_isLoopFadeIn(Live2DMotion *p) {
        return p->isLoopFadeIn();
    }

    Live2DMotion* Live2DMotion_loadMotion(const void *buf, int bufSize) {
        return Live2DMotion::loadMotion(buf, bufSize);
    }

    void Live2DMotion_dump(Live2DMotion *p) {
        p->dump();
    }

    void Live2DMotion_setParamFadeIn(Live2DMotion *p, const char *paramID, int value) {
        LDString paramIDStr(paramID);
        p->setParamFadeIn(paramIDStr, value);
    }

    void Live2DMotion_setParamFadeOut(Live2DMotion *p, const char *paramID, int value) {
        LDString paramIDStr(paramID);
        p->setParamFadeOut(paramIDStr, value);
    }

    int Live2DMotion_getParamFadeIn(Live2DMotion *p, const char *paramID) {
        return p->getParamFadeIn(paramID);
    }

    int Live2DMotion_getParamFadeOut(Live2DMotion *p, const char *paramID) {
        return p->getParamFadeOut(paramID);
    }
}