#ifndef CPY0I_RUNTIME_H
#define CPY0I_RUNTIME_H

#include "env.h"

typedef struct PyRuntime {
    PyEnv *globals;
    PyValue *sys_value;
} PyRuntime;

PyRuntime *runtime_new(void);
void runtime_set_argv(PyRuntime *rt, int argc, char **argv);
void runtime_init_builtins(PyRuntime *rt);

#endif
