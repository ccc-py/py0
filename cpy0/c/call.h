#ifndef CPY0I_CALL_H
#define CPY0I_CALL_H

#include "runtime.h"
#include "value.h"

PyValue *py_call(PyRuntime *rt, PyValue *callable, int argc, PyValue **argv);

#endif
