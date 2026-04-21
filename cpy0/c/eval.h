#ifndef CPY0I_EVAL_H
#define CPY0I_EVAL_H

#include "ast_nodes.h"
#include "env.h"
#include "exception.h"
#include "runtime.h"
#include "value.h"

PyValue *eval_expr(PyRuntime *rt, PyEnv *env, AstNode *node);
ExecResult exec_stmt(PyRuntime *rt, PyEnv *env, AstNode *node);
ExecResult exec_module(PyRuntime *rt, PyEnv *env, AstNode *module);
ExecResult exec_function_body(PyRuntime *rt, PyEnv *env, AstNode *function_def);

#endif
