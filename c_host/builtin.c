#include "builtin.h"

#include "ast.h"
#include "env.h"
#include "eval.h"
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
    if (argv[0]->type == PY_STR) {
        return py_new_int((long)strlen(argv[0]->as.s));
    }
    if (argv[0]->type == PY_LIST) {
        return py_new_int((long)argv[0]->as.list->count);
    }
    die("len() unsupported for this type");
    return py_none();
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

static PyValue *builtin_import(PyRuntime *rt, int argc, PyValue **argv) {
    (void)rt;
    if (argc != 1 || argv[0]->type != PY_STR) {
        die("__import__() expects one string argument");
    }
    if (strcmp(argv[0]->as.s, "sys") == 0) {
        return rt->sys_value;
    }
    die("unsupported import: %s", argv[0]->as.s);
    return py_none();
}

static char *read_file(const char *path) {
    FILE *fp = fopen(path, "rb");
    if (!fp) {
        die("cannot open %s", path);
    }
    fseek(fp, 0, SEEK_END);
    long size = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    char *buf = (char *)malloc((size_t)size + 1);
    if (!buf) {
        die("out of memory");
    }
    if (fread(buf, 1, (size_t)size, fp) != (size_t)size) {
        die("failed to read %s", path);
    }
    buf[size] = '\0';
    fclose(fp);
    return buf;
}

PyValue *builtin_run_path_impl(PyRuntime *rt, const char *path) {
    char *source = read_file(path);
    AstNode *module = parse_source(source, path);

    PyValue *old_argv = rt->sys_value->as.sys->argv;
    PyValue *new_argv = py_new_list();
    PyList *old_list = old_argv->as.list;
    for (int i = 1; i < old_list->count; i++) {
        py_list_append(new_argv, old_list->items[i]);
    }
    rt->sys_value->as.sys->argv = new_argv;

    PyEnv *module_env = env_new(rt->globals);
    env_set(module_env, "__name__", py_new_string("__main__"));
    env_set(module_env, "__file__", py_new_string(path));

    ExecResult result = exec_module(rt, module_env, module);
    (void)result;

    rt->sys_value->as.sys->argv = old_argv;
    ast_free(module);
    free(source);
    return py_none();
}

static PyValue *builtin_run_path(PyRuntime *rt, int argc, PyValue **argv) {
    if (argc != 1 || argv[0]->type != PY_STR) {
        die("run_path() expects one string argument");
    }
    return builtin_run_path_impl(rt, argv[0]->as.s);
}

void builtin_install(PyRuntime *rt) {
    env_set(rt->globals, "print", py_new_builtin("print", builtin_print));
    env_set(rt->globals, "len", py_new_builtin("len", builtin_len));
    env_set(rt->globals, "int", py_new_builtin("int", builtin_int));
    env_set(rt->globals, "float", py_new_builtin("float", builtin_float));
    env_set(rt->globals, "str", py_new_builtin("str", builtin_str));
    env_set(rt->globals, "bool", py_new_builtin("bool", builtin_bool));
    env_set(rt->globals, "__import__", py_new_builtin("__import__", builtin_import));
    env_set(rt->globals, "run_path", py_new_builtin("run_path", builtin_run_path));
}
