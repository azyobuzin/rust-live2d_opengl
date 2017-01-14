#include "w_common.h"
#include "ALive2DModel.h"

using namespace live2d;

extern "C" {
    float ALive2DModel_getParamFloat(ALive2DModel *p, const char *paramID) {
        return p->getParamFloat(paramID);
    }

    void ALive2DModel_setParamFloat(ALive2DModel *p, const char *paramID, float value, float weight) {
        p->setParamFloat(paramID, value, weight);
    }

    void ALive2DModel_addToParamFloat(ALive2DModel *p, const char *paramID, float value, float weight) {
        p->addToParamFloat(paramID, value, weight);
    }

    void ALive2DModel_multParamFloat(ALive2DModel *p, const char *paramID, float mult, float weight) {
        p->multParamFloat(paramID, mult, weight);
    }

    // TODO: inline members

    void ALive2DModel_loadParam(ALive2DModel *p) {
        p->loadParam();
    }

    void ALive2DModel_saveParam(ALive2DModel *p) {
        p->saveParam();
    }

    void ALive2DModel_init(ALive2DModel *p) {
        p->init();
    }

    void ALive2DModel_update(ALive2DModel *p) {
        p->update();
    }

    void ALive2DModel_draw(ALive2DModel *p) {
        p->draw();
    }

    void ALive2DModel_setPartsOpacityByID(ALive2DModel *p, const char *partsID, float opacity) {
        p->setPartsOpacity(partsID, opacity);
    }

    void ALive2DModel_setPartsOpacityByIndex(ALive2DModel *p, int partsIndex, float opacity) {
        p->setPartsOpacity(partsIndex, opacity);
    }

    float ALive2DModel_getPartsOpacityByID(ALive2DModel *p, const char *partsID) {
        return p->getPartsOpacity(partsID);
    }

    float ALive2DModel_getPartsOpacityByIndex(ALive2DModel *p, int partsIndex) {
        return p->getPartsOpacity(partsIndex);
    }

    void ALive2DModel_setupPartsOpacityGroup_alphaImpl(ALive2DModel *p, const char *paramGroup[], float deltaTimeSec) {
        p->setupPartsOpacityGroup_alphaImpl(paramGroup, deltaTimeSec);
    }

    void ALive2DModel_setModelImpl(ALive2DModel *p, ModelImpl *m) {
        p->setModelImpl(m);
    }

    ModelImpl* ALive2DModel_getModelImpl(ALive2DModel *p) {
        return p->getModelImpl();
    }

    ModelContext* ALive2DModel_getModelContext(ALive2DModel *p) {
        return p->getModelContext();
    }

    int ALive2DModel_getErrorFlags(ALive2DModel *p) {
        return p->getErrorFlags();
    }

    int ALive2DModel_generateModelTextureNo(ALive2DModel *p) {
        return p->generateModelTextureNo();
    }

    void ALive2DModel_releaseModelTextureNo(ALive2DModel *p, int no) {
        p->releaseModelTextureNo(no);
    }

    float ALive2DModel_getCanvasWidth(ALive2DModel *p) {
        return p->getCanvasWidth();
    }

    float ALive2DModel_getCanvasHeight(ALive2DModel *p) {
        return p->getCanvasHeight();
    }

    DrawParam* ALive2DModel_getDrawParam(ALive2DModel *p) {
        return p->getDrawParam();
    }

    int ALive2DModel_getDrawDataIndex(ALive2DModel *p, const char *drawDataID) {
        return p->getDrawDataIndex(drawDataID);
    }

    IDrawData* ALive2DModel_getDrawData(ALive2DModel *p, int drawIndex) {
        return p->getDrawData(drawIndex);
    }

    l2d_pointf* ALive2DModel_getTransformedPoints(ALive2DModel *p, int drawIndex, int *pointCount) {
        return p->getTransformedPoints(drawIndex, pointCount);
    }

    l2d_index* ALive2DModel_getIndexArray(ALive2DModel *p, int drawIndex, int *polygonCount) {
        return p->getIndexArray(drawIndex, polygonCount);
    }

    void ALive2DModel_updateZBuffer_TestImpl(ALive2DModel *p, float startZ, float stepZ) {
        p->updateZBuffer_TestImpl(startZ, stepZ);
    }

    // TODO: inline members

    void ALive2DModel_setPremultipliedAlpha(ALive2DModel *p, w_bool b) {
        p->setPremultipliedAlpha(b);
    }

    w_bool ALive2DModel_isPremultipliedAlpha(ALive2DModel *p) {
        return p->isPremultipliedAlpha();
    }

    void ALive2DModel_setAnisotropy(ALive2DModel *p, int n) {
        p->setAnisotropy(n);
    }

    int ALive2DModel_getAnisotropy(ALive2DModel *p) {
        return p->getAnisotropy();
    }
}
