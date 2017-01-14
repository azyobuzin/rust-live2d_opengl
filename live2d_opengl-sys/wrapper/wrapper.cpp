#pragma comment(lib, "opengl32.lib")
#include "w_common.h"
#include "memory/LDObject.h"

using namespace live2d;

extern "C" {
    void delete_ld_object(LDObject *obj) {
        delete obj;
    }
}
