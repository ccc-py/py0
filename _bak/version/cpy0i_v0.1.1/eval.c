#include "eval.h"

#include "call.h"
#include "function.h"
#include "operator.h"

#include <stdlib.h>

static ExecResult exec_block(PyRuntime *rt, PyEnv *env, PtrVec *body) {
    ExecResult result = {0, NULL};
    for (int i = 0; i < body->count; i++) {
        result = exec_stmt(rt, env, (AstNode *)body->items[i]);
        if (result.has_return) {
            return result;
        }
    }
    return result;
}

PyValue *eval_expr(PyRuntime *rt, PyEnv *env, AstNode *node) {
    (void)rt;
    switch (node->kind) {
        case AST_NAME:
            return env_get(env, node->as.name.name);
        case AST_CONSTANT:
            if (node->as.constant.is_string) {
                return py_new_string(node->as.constant.str_value);
            }
            if (node->as.constant.is_float) {
                return py_new_float(node->as.constant.float_value);
            }
            return py_new_int(node->as.constant.int_value);
        case AST_BINOP: {
            PyValue *left = eval_expr(rt, env, node->as.binop.left);
            PyValue *right = eval_expr(rt, env, node->as.binop.right);
            return py_apply_binop(node->as.binop.op, left, right);
        }
        case AST_UNARYOP: {
            PyValue *value = eval_expr(rt, env, node->as.unaryop.operand);
            return py_apply_unary(node->as.unaryop.op, value);
        }
        case AST_COMPARE: {
            PyValue *left = eval_expr(rt, env, node->as.compare.left);
            PyValue *right = eval_expr(rt, env, node->as.compare.right);
            return py_apply_compare(node->as.compare.op, left, right);
        }
        case AST_CALL: {
            PyValue *func = eval_expr(rt, env, node->as.call.func);
            int argc = node->as.call.args.count;
            PyValue **argv = argc ? (PyValue **)calloc((size_t)argc, sizeof(PyValue *)) : NULL;
            for (int i = 0; i < argc; i++) {
                argv[i] = eval_expr(rt, env, (AstNode *)node->as.call.args.items[i]);
            }
            PyValue *result = py_call(rt, func, argc, argv);
            free(argv);
            return result;
        }
        default:
            die("unsupported expression kind %d", node->kind);
    }
    return py_none();
}

ExecResult exec_stmt(PyRuntime *rt, PyEnv *env, AstNode *node) {
    ExecResult result = {0, NULL};
    switch (node->kind) {
        case AST_EXPR_STMT:
            (void)eval_expr(rt, env, node->as.expr_stmt.expr);
            return result;
        case AST_ASSIGN: {
            PyValue *value = eval_expr(rt, env, node->as.assign.value);
            env_assign(env, node->as.assign.name, value);
            return result;
        }
        case AST_IF: {
            PyValue *cond = eval_expr(rt, env, node->as.if_stmt.test);
            if (py_is_truthy(cond)) {
                return exec_block(rt, env, &node->as.if_stmt.body);
            }
            return exec_block(rt, env, &node->as.if_stmt.orelse);
        }
        case AST_WHILE:
            while (py_is_truthy(eval_expr(rt, env, node->as.while_stmt.test))) {
                result = exec_block(rt, env, &node->as.while_stmt.body);
                if (result.has_return) {
                    return result;
                }
            }
            return result;
        case AST_FUNCTION_DEF: {
            PyValue *fn = py_make_function(
                node->as.function_def.name,
                node->as.function_def.params,
                node->as.function_def.param_count,
                node,
                env
            );
            env_set(env, node->as.function_def.name, fn);
            return result;
        }
        case AST_RETURN:
            result.has_return = 1;
            result.value = node->as.return_stmt.value ? eval_expr(rt, env, node->as.return_stmt.value) : py_none();
            return result;
        case AST_PASS:
            return result;
        default:
            die("unsupported statement kind %d", node->kind);
    }
    return result;
}

ExecResult exec_module(PyRuntime *rt, PyEnv *env, AstNode *module) {
    if (module->kind != AST_MODULE) {
        die("expected module");
    }
    return exec_block(rt, env, &module->as.module.body);
}

ExecResult exec_function_body(PyRuntime *rt, PyEnv *env, AstNode *function_def) {
    if (function_def->kind != AST_FUNCTION_DEF) {
        die("expected function definition");
    }
    return exec_block(rt, env, &function_def->as.function_def.body);
}
