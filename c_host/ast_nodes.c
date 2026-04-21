#include "ast_nodes.h"

#include <stdlib.h>

static AstNode *ast_alloc(AstKind kind, int line) {
    AstNode *node = (AstNode *)calloc(1, sizeof(AstNode));
    if (!node) {
        die("out of memory");
    }
    node->kind = kind;
    node->line = line;
    return node;
}

AstNode *ast_new_module(void) {
    AstNode *node = ast_alloc(AST_MODULE, 1);
    ptrvec_init(&node->as.module.body);
    return node;
}

AstNode *ast_new_expr_stmt(AstNode *expr, int line) {
    AstNode *node = ast_alloc(AST_EXPR_STMT, line);
    node->as.expr_stmt.expr = expr;
    return node;
}

AstNode *ast_new_assign(char *name, AstNode *value, int line) {
    AstNode *node = ast_alloc(AST_ASSIGN, line);
    node->as.assign.name = name;
    node->as.assign.value = value;
    return node;
}

AstNode *ast_new_if(AstNode *test, int line) {
    AstNode *node = ast_alloc(AST_IF, line);
    node->as.if_stmt.test = test;
    ptrvec_init(&node->as.if_stmt.body);
    ptrvec_init(&node->as.if_stmt.orelse);
    return node;
}

AstNode *ast_new_while(AstNode *test, int line) {
    AstNode *node = ast_alloc(AST_WHILE, line);
    node->as.while_stmt.test = test;
    ptrvec_init(&node->as.while_stmt.body);
    return node;
}

AstNode *ast_new_function_def(char *name, char **params, int param_count, int line) {
    AstNode *node = ast_alloc(AST_FUNCTION_DEF, line);
    node->as.function_def.name = name;
    node->as.function_def.params = params;
    node->as.function_def.param_count = param_count;
    ptrvec_init(&node->as.function_def.body);
    return node;
}

AstNode *ast_new_return(AstNode *value, int line) {
    AstNode *node = ast_alloc(AST_RETURN, line);
    node->as.return_stmt.value = value;
    return node;
}

AstNode *ast_new_pass(int line) {
    return ast_alloc(AST_PASS, line);
}

AstNode *ast_new_name(char *name, int line) {
    AstNode *node = ast_alloc(AST_NAME, line);
    node->as.name.name = name;
    return node;
}

AstNode *ast_new_int(long value, int line) {
    AstNode *node = ast_alloc(AST_CONSTANT, line);
    node->as.constant.int_value = value;
    return node;
}

AstNode *ast_new_float(double value, int line) {
    AstNode *node = ast_alloc(AST_CONSTANT, line);
    node->as.constant.is_float = 1;
    node->as.constant.float_value = value;
    return node;
}

AstNode *ast_new_string(char *value, int line) {
    AstNode *node = ast_alloc(AST_CONSTANT, line);
    node->as.constant.is_string = 1;
    node->as.constant.str_value = value;
    return node;
}

AstNode *ast_new_binop(OpKind op, AstNode *left, AstNode *right, int line) {
    AstNode *node = ast_alloc(AST_BINOP, line);
    node->as.binop.op = op;
    node->as.binop.left = left;
    node->as.binop.right = right;
    return node;
}

AstNode *ast_new_unaryop(OpKind op, AstNode *operand, int line) {
    AstNode *node = ast_alloc(AST_UNARYOP, line);
    node->as.unaryop.op = op;
    node->as.unaryop.operand = operand;
    return node;
}

AstNode *ast_new_compare(OpKind op, AstNode *left, AstNode *right, int line) {
    AstNode *node = ast_alloc(AST_COMPARE, line);
    node->as.compare.op = op;
    node->as.compare.left = left;
    node->as.compare.right = right;
    return node;
}

AstNode *ast_new_call(AstNode *func, int line) {
    AstNode *node = ast_alloc(AST_CALL, line);
    node->as.call.func = func;
    ptrvec_init(&node->as.call.args);
    return node;
}

AstNode *ast_new_attribute(AstNode *value, char *attr, int line) {
    AstNode *node = ast_alloc(AST_ATTRIBUTE, line);
    node->as.attribute.value = value;
    node->as.attribute.attr = attr;
    return node;
}

AstNode *ast_new_subscript(AstNode *value, AstNode *index, int line) {
    AstNode *node = ast_alloc(AST_SUBSCRIPT, line);
    node->as.subscript.value = value;
    node->as.subscript.index = index;
    return node;
}

static void ast_free_vec(PtrVec *vec) {
    for (int i = 0; i < vec->count; i++) {
        ast_free((AstNode *)vec->items[i]);
    }
    ptrvec_free(vec);
}

void ast_free(AstNode *node) {
    if (!node) {
        return;
    }
    switch (node->kind) {
        case AST_MODULE:
            ast_free_vec(&node->as.module.body);
            break;
        case AST_EXPR_STMT:
            ast_free(node->as.expr_stmt.expr);
            break;
        case AST_ASSIGN:
            free(node->as.assign.name);
            ast_free(node->as.assign.value);
            break;
        case AST_IF:
            ast_free(node->as.if_stmt.test);
            ast_free_vec(&node->as.if_stmt.body);
            ast_free_vec(&node->as.if_stmt.orelse);
            break;
        case AST_WHILE:
            ast_free(node->as.while_stmt.test);
            ast_free_vec(&node->as.while_stmt.body);
            break;
        case AST_FUNCTION_DEF:
            free(node->as.function_def.name);
            for (int i = 0; i < node->as.function_def.param_count; i++) {
                free(node->as.function_def.params[i]);
            }
            free(node->as.function_def.params);
            ast_free_vec(&node->as.function_def.body);
            break;
        case AST_RETURN:
            ast_free(node->as.return_stmt.value);
            break;
        case AST_NAME:
            free(node->as.name.name);
            break;
        case AST_CONSTANT:
            if (node->as.constant.is_string) {
                free(node->as.constant.str_value);
            }
            break;
        case AST_BINOP:
            ast_free(node->as.binop.left);
            ast_free(node->as.binop.right);
            break;
        case AST_UNARYOP:
            ast_free(node->as.unaryop.operand);
            break;
        case AST_COMPARE:
            ast_free(node->as.compare.left);
            ast_free(node->as.compare.right);
            break;
        case AST_CALL:
            ast_free(node->as.call.func);
            ast_free_vec(&node->as.call.args);
            break;
        case AST_ATTRIBUTE:
            ast_free(node->as.attribute.value);
            free(node->as.attribute.attr);
            break;
        case AST_SUBSCRIPT:
            ast_free(node->as.subscript.value);
            ast_free(node->as.subscript.index);
            break;
        case AST_PASS:
            break;
    }
    free(node);
}
