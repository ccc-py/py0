#include "builtin.h"

#include "env.h"
#include "util.h"
#include "value.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static PyValue *builtin_print(PyRuntime *rt, int argc, PyValue **argv) {
    (void)rt;
    for (int i = 0; i < argc; i++) {
        char *text = py_to_string(argv[i]);
        if (i > 0) {
            fputc(' ', stdout);
        }
        fputs(text, stdout);
        free(text);
    }
    fputc('\n', stdout);
    return py_none();
}

static PyValue *builtin_len(PyRuntime *rt, int argc, PyValue **argv) {
    (void)rt;
    if (argc != 1) {
        die("len() expects 1 argument");
    }
    if (argv[0]->type != PY_STR) {
        die("len() currently supports strings only");
    }
    return py_new_int((long)strlen(argv[0]->as.s));
}

static PyValue *builtin_int(PyRuntime *rt, int argc, PyValue **argv) {
    (void)rt;
    if (argc != 1) die("int() expects 1 argument");
    if (argv[0]->type == PY_INT) return argv[0];
    if (argv[0]->type == PY_FLOAT) return py_new_int((long)argv[0]->as.f);
    if (argv[0]->type == PY_BOOL) return py_new_int((long)argv[0]->as.b);
    die("int() unsupported for this type");
    return py_none();
}

static PyValue *builtin_float(PyRuntime *rt, int argc, PyValue **argv) {
    (void)rt;
    if (argc != 1) die("float() expects 1 argument");
    return py_new_float(py_as_number(argv[0]));
}

static PyValue *builtin_str(PyRuntime *rt, int argc, PyValue **argv) {
    (void)rt;
    if (argc != 1) die("str() expects 1 argument");
    char *s = py_to_string(argv[0]);
    PyValue *out = py_new_string(s);
    free(s);
    return out;
}

static PyValue *builtin_bool(PyRuntime *rt, int argc, PyValue **argv) {
    (void)rt;
    if (argc != 1) die("bool() expects 1 argument");
    return py_bool(py_is_truthy(argv[0]));
}

void builtin_install(PyRuntime *rt) {
    env_set(rt->globals, "print", py_new_builtin("print", builtin_print));
    env_set(rt->globals, "len", py_new_builtin("len", builtin_len));
    env_set(rt->globals, "int", py_new_builtin("int", builtin_int));
    env_set(rt->globals, "float", py_new_builtin("float", builtin_float));
    env_set(rt->globals, "str", py_new_builtin("str", builtin_str));
    env_set(rt->globals, "bool", py_new_builtin("bool", builtin_bool));
}
