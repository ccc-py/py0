#include "runtime.h"

#include "builtin.h"
#include "util.h"

#include <stdlib.h>

PyRuntime *runtime_new(void) {
    PyRuntime *rt = (PyRuntime *)calloc(1, sizeof(PyRuntime));
    if (!rt) die("out of memory");
    rt->globals = env_new(NULL);
    runtime_init_builtins(rt);
    return rt;
}

void runtime_set_argv(PyRuntime *rt, int argc, char **argv) {
    PyValue *list = py_new_list();
    for (int i = 0; i < argc; i++) {
        py_list_append(list, py_new_string(argv[i]));
    }
    rt->sys_value = py_new_sys(list);
    env_set(rt->globals, "sys", rt->sys_value);
}

void runtime_init_builtins(PyRuntime *rt) {
    builtin_install(rt);
}
