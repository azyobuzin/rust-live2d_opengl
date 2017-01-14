#include "w_common.h"
#include "Live2D.h"

using namespace live2d;

extern "C" {
    int Live2D_getClippingMaskBufferSize() {
        return Live2D::getClippingMaskBufferSize();
    }

    void Live2D_setClippingMaskBufferSize(int size) {
        Live2D::setClippingMaskBufferSize(size);
    }

    void Live2D_init() {
        Live2D::init();
    }

    void Live2D_dispose() {
        Live2D::dispose();
    }

    const char *Live2D_getVersionStr() {
        return Live2D::getVersionStr();
    }

    l2d_uint32 Live2D_getVersionNo() {
        return Live2D::getVersionNo();
    }

    w_bool Live2D_getBuildOption_RANGE_CHECK_POINT() {
        return Live2D::getBuildOption_RANGE_CHECK_POINT();
    }

    w_bool Live2D_getBuildOption_AVATAR_OPTION_A() {
        return Live2D::getBuildOption_AVATAR_OPTION_A();
    }

    void Live2D_setVertexDoubleBufferEnabled(w_bool enabled) {
        Live2D::setVertexDoubleBufferEnabled(enabled);
    }

    w_bool Live2D_isVertexDoubleBufferEnabled() {
        return Live2D::isVertexDoubleBufferEnabled();
    }

    void Live2D_setError(l2d_uint32 errorNo) {
        Live2D::setError(errorNo);
    }

    l2d_uint32 Live2D_getError() {
        return Live2D::getError();
    }
}
