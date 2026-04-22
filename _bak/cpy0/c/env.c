#include "env.h"

#include "util.h"

#include <stdlib.h>
#include <string.h>

PyEnv *env_new(PyEnv *parent) {
    PyEnv *env = (PyEnv *)calloc(1, sizeof(PyEnv));
    if (!env) die("out of memory");
    env->parent = parent;
    return env;
}

static int env_find_local(PyEnv *env, const char *name) {
    for (int i = 0; i < env->count; i++) {
        if (strcmp(env->items[i].name, name) == 0) {
            return i;
        }
    }
    return -1;
}

void env_set(PyEnv *env, const char *name, PyValue *value) {
    int idx = env_find_local(env, name);
    if (idx >= 0) {
        env->items[idx].value = value;
        return;
    }
    if (env->count == env->capacity) {
        int new_cap = env->capacity ? env->capacity * 2 : 8;
        PyBinding *new_items = (PyBinding *)realloc(env->items, sizeof(PyBinding) * new_cap);
        if (!new_items) die("out of memory");
        env->items = new_items;
        env->capacity = new_cap;
    }
    env->items[env->count].name = xstrdup(name);
    env->items[env->count].value = value;
    env->count++;
}

void env_assign(PyEnv *env, const char *name, PyValue *value) {
    for (PyEnv *cur = env; cur; cur = cur->parent) {
        int idx = env_find_local(cur, name);
        if (idx >= 0) {
            cur->items[idx].value = value;
            return;
        }
    }
    env_set(env, name, value);
}

PyValue *env_get(PyEnv *env, const char *name) {
    for (PyEnv *cur = env; cur; cur = cur->parent) {
        int idx = env_find_local(cur, name);
        if (idx >= 0) {
            return cur->items[idx].value;
        }
    }
    die("name '%s' is not defined", name);
    return NULL;
}
