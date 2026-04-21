#include "operator.h"

#include "util.h"

#include <stdlib.h>
#include <string.h>

static int both_ints(PyValue *a, PyValue *b) {
    return a->type == PY_INT && b->type == PY_INT;
}

PyValue *py_apply_binop(OpKind op, PyValue *left, PyValue *right) {
    if (op == OP_ADD && left->type == PY_STR && right->type == PY_STR) {
        StrBuf buf;
        strbuf_init(&buf);
        strbuf_append_str(&buf, left->as.s);
        strbuf_append_str(&buf, right->as.s);
        char *joined = strbuf_take(&buf);
        PyValue *out = py_new_string(joined);
        free(joined);
        return out;
    }

    if (both_ints(left, right)) {
        long a = left->as.i;
        long b = right->as.i;
        switch (op) {
            case OP_ADD: return py_new_int(a + b);
            case OP_SUB: return py_new_int(a - b);
            case OP_MUL: return py_new_int(a * b);
            case OP_DIV: return py_new_float((double)a / (double)b);
            case OP_MOD: return py_new_int(a % b);
            default: break;
        }
    }

    double a = py_as_number(left);
    double b = py_as_number(right);
    switch (op) {
        case OP_ADD: return py_new_float(a + b);
        case OP_SUB: return py_new_float(a - b);
        case OP_MUL: return py_new_float(a * b);
        case OP_DIV: return py_new_float(a / b);
        case OP_MOD: return py_new_float((long)a % (long)b);
        default:
            die("unsupported binary operator");
    }
    return py_none();
}

PyValue *py_apply_compare(OpKind op, PyValue *left, PyValue *right) {
    if (left->type == PY_STR && right->type == PY_STR) {
        int cmp = strcmp(left->as.s, right->as.s);
        switch (op) {
            case OP_EQ: return py_bool(cmp == 0);
            case OP_NE: return py_bool(cmp != 0);
            case OP_LT: return py_bool(cmp < 0);
            case OP_LE: return py_bool(cmp <= 0);
            case OP_GT: return py_bool(cmp > 0);
            case OP_GE: return py_bool(cmp >= 0);
            default: break;
        }
    }

    double a = py_as_number(left);
    double b = py_as_number(right);
    switch (op) {
        case OP_EQ: return py_bool(a == b);
        case OP_NE: return py_bool(a != b);
        case OP_LT: return py_bool(a < b);
        case OP_LE: return py_bool(a <= b);
        case OP_GT: return py_bool(a > b);
        case OP_GE: return py_bool(a >= b);
        default:
            die("unsupported comparison operator");
    }
    return py_bool(0);
}

PyValue *py_apply_unary(OpKind op, PyValue *value) {
    switch (op) {
        case OP_NEG:
            if (value->type == PY_INT) {
                return py_new_int(-value->as.i);
            }
            return py_new_float(-py_as_number(value));
        default:
            die("unsupported unary operator");
    }
    return py_none();
}
