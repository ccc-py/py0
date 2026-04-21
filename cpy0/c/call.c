#include "call.h"

#include "eval.h"
#include "util.h"

PyValue *py_call(PyRuntime *rt, PyValue *callable, int argc, PyValue **argv) {
    if (callable->type == PY_BUILTIN_FUNCTION) {
        return callable->as.cfunc->fn(rt, argc, argv);
    }
    if (callable->type == PY_FUNCTION) {
        PyFunction *fn = callable->as.func;
        if (argc != fn->param_count) {
            die("%s() expected %d arguments, got %d", fn->name, fn->param_count, argc);
        }
        PyEnv *local = env_new(fn->closure);
        for (int i = 0; i < argc; i++) {
            env_set(local, fn->params[i], argv[i]);
        }
        ExecResult result = exec_function_body(rt, local, fn->body);
        if (result.has_return) {
            return result.value;
        }
        return py_none();
    }
    die("object is not callable");
    return py_none();
}
