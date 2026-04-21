#ifndef CPY0I_OPERATOR_H
#define CPY0I_OPERATOR_H

#include "value.h"
#include "ast_nodes.h"

PyValue *py_apply_binop(OpKind op, PyValue *left, PyValue *right);
PyValue *py_apply_compare(OpKind op, PyValue *left, PyValue *right);
PyValue *py_apply_unary(OpKind op, PyValue *value);

#endif
