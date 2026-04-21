#ifndef CPY0I_RUNTIME_H
#define CPY0I_RUNTIME_H

#include "env.h"

typedef struct PyRuntime {
    PyEnv *globals;
} PyRuntime;

PyRuntime *runtime_new(void);
void runtime_init_builtins(PyRuntime *rt);

#endif
