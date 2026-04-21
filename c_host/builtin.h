#ifndef CPY0I_BUILTIN_H
#define CPY0I_BUILTIN_H

#include "runtime.h"

void builtin_install(struct PyRuntime *rt);
PyValue *builtin_run_path_impl(struct PyRuntime *rt, const char *path);

#endif
