#include "../w_common.h"
#include "motion/AMotion.h"

using namespace live2d;

extern "C" {
    void AMotion_updateParam(AMotion *p, ALive2DModel *model, MotionQueueEnt *motionQueueEnt) {
        p->updateParam(model, motionQueueEnt);
    }

    void AMotion_setFadeIn(AMotion *p, int fadeInMsec) {
        p->setFadeIn(fadeInMsec);
    }

    void AMotion_setFadeOut(AMotion *p, int fadeOutMsec) {
        p->setFadeOut(fadeOutMsec);
    }

    int AMotion_getFadeOut(AMotion *p) {
        return p->getFadeOut();
    }

    int AMotion_getFadeIn(AMotion *p) {
        return p->getFadeIn();
    }

    void AMotion_setWeight(AMotion *p, float weight) {
        p->setWeight(weight);
    }

    float AMotion_getWeight(AMotion *p) {
        return p->getWeight();
    }

    int AMotion_getDurationMSec(AMotion *p) {
        return p->getDurationMSec();
    }

    int AMotion_getLoopDurationMSec(AMotion *p) {
        return p->getLoopDurationMSec();
    }

    void AMotion_setOffsetMSec(AMotion *p, int offsetMsec) {
        p->setOffsetMSec(offsetMsec);
    }
}
