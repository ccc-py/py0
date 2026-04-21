#include "function.h"

#include "util.h"

#include <stdlib.h>

PyValue *py_make_function(const char *name, char **params, int param_count, AstNode *body, PyEnv *closure) {
    PyFunction *fn = (PyFunction *)calloc(1, sizeof(PyFunction));
    if (!fn) die("out of memory");
    fn->name = xstrdup(name);
    fn->params = params;
    fn->param_count = param_count;
    fn->body = body;
    fn->closure = closure;
    return py_new_function(fn);
}
