#include "w_common.h"
#include "Live2DModelWinGL.h"

using namespace live2d;

extern "C" {
    void Live2DModelWinGL_setTexture(Live2DModelWinGL *p, int textureNo, unsigned int openGLTextureNo) {
        p->setTexture(textureNo, openGLTextureNo);
    }

    Live2DModelWinGL* Live2DModelWinGL_loadModel(const void* buf, int bufSize) {
        return Live2DModelWinGL::loadModel(buf, bufSize);
    }

    void Live2DModelWinGL_setMatrix(Live2DModelWinGL *p, float *matrix) {
        p->setMatrix(matrix);
    }
}
