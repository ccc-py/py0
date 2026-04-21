#ifndef CPY0I_ENV_H
#define CPY0I_ENV_H

#include "value.h"

typedef struct {
    char *name;
    PyValue *value;
} PyBinding;

typedef struct PyEnv {
    struct PyEnv *parent;
    PyBinding *items;
    int count;
    int capacity;
} PyEnv;

PyEnv *env_new(PyEnv *parent);
void env_set(PyEnv *env, const char *name, PyValue *value);
void env_assign(PyEnv *env, const char *name, PyValue *value);
PyValue *env_get(PyEnv *env, const char *name);

#endif
