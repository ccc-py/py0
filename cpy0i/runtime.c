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

void runtime_init_builtins(PyRuntime *rt) {
    builtin_install(rt);
}
