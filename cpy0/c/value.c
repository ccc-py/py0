#include "value.h"

#include "util.h"

#include <stdio.h>
#include <stdlib.h>

PyValue PY_NONE_VALUE = { PY_NONE, {0} };
PyValue PY_TRUE_VALUE = { PY_BOOL, {.b = 1} };
PyValue PY_FALSE_VALUE = { PY_BOOL, {.b = 0} };

PyValue *py_none(void) {
    return &PY_NONE_VALUE;
}

PyValue *py_bool(int value) {
    return value ? &PY_TRUE_VALUE : &PY_FALSE_VALUE;
}

PyValue *py_new_int(long value) {
    PyValue *v = (PyValue *)calloc(1, sizeof(PyValue));
    if (!v) die("out of memory");
    v->type = PY_INT;
    v->as.i = value;
    return v;
}

PyValue *py_new_float(double value) {
    PyValue *v = (PyValue *)calloc(1, sizeof(PyValue));
    if (!v) die("out of memory");
    v->type = PY_FLOAT;
    v->as.f = value;
    return v;
}

PyValue *py_new_string(const char *value) {
    PyValue *v = (PyValue *)calloc(1, sizeof(PyValue));
    if (!v) die("out of memory");
    v->type = PY_STR;
    v->as.s = xstrdup(value);
    return v;
}

PyValue *py_new_list(void) {
    PyValue *v = (PyValue *)calloc(1, sizeof(PyValue));
    PyList *list = (PyList *)calloc(1, sizeof(PyList));
    if (!v || !list) die("out of memory");
    v->type = PY_LIST;
    v->as.list = list;
    return v;
}

void py_list_append(PyValue *list_value, PyValue *item) {
    if (list_value->type != PY_LIST) {
        die("expected list");
    }
    PyList *list = list_value->as.list;
    if (list->count == list->capacity) {
        int new_cap = list->capacity ? list->capacity * 2 : 8;
        PyValue **new_items = (PyValue **)realloc(list->items, sizeof(PyValue *) * new_cap);
        if (!new_items) die("out of memory");
        list->items = new_items;
        list->capacity = new_cap;
    }
    list->items[list->count++] = item;
}

PyValue *py_list_get(PyValue *list_value, long index) {
    if (list_value->type != PY_LIST) {
        die("expected list");
    }
    PyList *list = list_value->as.list;
    if (index < 0 || index >= list->count) {
        die("list index out of range");
    }
    return list->items[index];
}

PyValue *py_new_sys(PyValue *argv_list) {
    PyValue *v = (PyValue *)calloc(1, sizeof(PyValue));
    PySys *sys = (PySys *)calloc(1, sizeof(PySys));
    if (!v || !sys) die("out of memory");
    v->type = PY_SYS;
    sys->argv = argv_list;
    v->as.sys = sys;
    return v;
}

PyValue *py_new_function(PyFunction *func) {
    PyValue *v = (PyValue *)calloc(1, sizeof(PyValue));
    if (!v) die("out of memory");
    v->type = PY_FUNCTION;
    v->as.func = func;
    return v;
}

PyValue *py_new_builtin(const char *name, PyCFunction fn) {
    PyValue *v = (PyValue *)calloc(1, sizeof(PyValue));
    PyBuiltinFunction *cfunc = (PyBuiltinFunction *)calloc(1, sizeof(PyBuiltinFunction));
    if (!v || !cfunc) die("out of memory");
    cfunc->name = xstrdup(name);
    cfunc->fn = fn;
    v->type = PY_BUILTIN_FUNCTION;
    v->as.cfunc = cfunc;
    return v;
}

int py_is_truthy(PyValue *value) {
    switch (value->type) {
        case PY_NONE: return 0;
        case PY_BOOL: return value->as.b;
        case PY_INT: return value->as.i != 0;
        case PY_FLOAT: return value->as.f != 0.0;
        case PY_STR: return value->as.s[0] != '\0';
        case PY_LIST: return value->as.list->count != 0;
        case PY_SYS:
        case PY_FUNCTION:
        case PY_BUILTIN_FUNCTION:
            return 1;
    }
    return 0;
}

double py_as_number(PyValue *value) {
    switch (value->type) {
        case PY_INT: return (double)value->as.i;
        case PY_FLOAT: return value->as.f;
        case PY_BOOL: return (double)value->as.b;
        default:
            die("expected number");
    }
    return 0.0;
}

char *py_to_string(PyValue *value) {
    char buf[128];
    switch (value->type) {
        case PY_NONE:
            return xstrdup("None");
        case PY_BOOL:
            return xstrdup(value->as.b ? "True" : "False");
        case PY_INT:
            snprintf(buf, sizeof(buf), "%ld", value->as.i);
            return xstrdup(buf);
        case PY_FLOAT:
            snprintf(buf, sizeof(buf), "%.15g", value->as.f);
            return xstrdup(buf);
        case PY_STR:
            return xstrdup(value->as.s);
        case PY_LIST:
            return xstrdup("<list>");
        case PY_SYS:
            return xstrdup("<sys>");
        case PY_FUNCTION:
            return xstrdup("<function>");
        case PY_BUILTIN_FUNCTION:
            return xstrdup("<builtin-function>");
    }
    return xstrdup("<unknown>");
}
