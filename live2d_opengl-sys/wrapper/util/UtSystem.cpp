#include "../w_common.h"
#include "util/UtSystem.h"

using namespace live2d;

extern "C" {
    w_bool UtSystem_isBigEndian() {
        return UtSystem::isBigEndian();
    }

    l2d_int64 UtSystem_getTimeMSec() {
        return UtSystem::getTimeMSec();
    }

    l2d_int64 UtSystem_getUserTimeMSec() {
        return UtSystem::getUserTimeMSec();
    }

    void UtSystem_setUserTimeMSec(l2d_int64 t) {
        UtSystem::setUserTimeMSec(t);
    }

    l2d_int64 UtSystem_updateUserTimeMSec() {
        return UtSystem::updateUserTimeMSec();
    }

    void UtSystem_resetUserTimeMSec() {
        UtSystem::resetUserTimeMSec();
    }
}
