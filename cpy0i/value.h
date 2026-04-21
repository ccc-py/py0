#ifndef CPY0I_VALUE_H
#define CPY0I_VALUE_H

struct AstNode;
struct PyEnv;
struct PyRuntime;

typedef enum {
    PY_NONE,
    PY_BOOL,
    PY_INT,
    PY_FLOAT,
    PY_STR,
    PY_FUNCTION,
    PY_BUILTIN_FUNCTION
} PyType;

typedef struct PyValue PyValue;
typedef PyValue *(*PyCFunction)(struct PyRuntime *rt, int argc, PyValue **argv);

typedef struct {
    char *name;
    char **params;
    int param_count;
    struct AstNode *body;
    struct PyEnv *closure;
} PyFunction;

typedef struct {
    char *name;
    PyCFunction fn;
} PyBuiltinFunction;

struct PyValue {
    PyType type;
    union {
        int b;
        long i;
        double f;
        char *s;
        PyFunction *func;
        PyBuiltinFunction *cfunc;
    } as;
};

extern PyValue PY_NONE_VALUE;
extern PyValue PY_TRUE_VALUE;
extern PyValue PY_FALSE_VALUE;

PyValue *py_none(void);
PyValue *py_bool(int value);
PyValue *py_new_int(long value);
PyValue *py_new_float(double value);
PyValue *py_new_string(const char *value);
PyValue *py_new_function(PyFunction *func);
PyValue *py_new_builtin(const char *name, PyCFunction fn);

int py_is_truthy(PyValue *value);
double py_as_number(PyValue *value);
char *py_to_string(PyValue *value);

#endif
