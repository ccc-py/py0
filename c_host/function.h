#ifndef CPY0I_FUNCTION_H
#define CPY0I_FUNCTION_H

#include "ast_nodes.h"
#include "env.h"
#include "value.h"

PyValue *py_make_function(const char *name, char **params, int param_count, AstNode *body, PyEnv *closure);

#endif
