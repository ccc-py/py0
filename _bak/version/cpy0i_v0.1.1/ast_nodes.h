#ifndef CPY0I_AST_NODES_H
#define CPY0I_AST_NODES_H

#include "util.h"

typedef struct AstNode AstNode;

typedef enum {
    AST_MODULE,
    AST_EXPR_STMT,
    AST_ASSIGN,
    AST_IF,
    AST_WHILE,
    AST_FUNCTION_DEF,
    AST_RETURN,
    AST_PASS,
    AST_NAME,
    AST_CONSTANT,
    AST_BINOP,
    AST_UNARYOP,
    AST_COMPARE,
    AST_CALL
} AstKind;

typedef enum {
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,
    OP_MOD,
    OP_EQ,
    OP_NE,
    OP_LT,
    OP_LE,
    OP_GT,
    OP_GE,
    OP_NEG
} OpKind;

struct AstNode {
    AstKind kind;
    int line;
    union {
        struct {
            PtrVec body;
        } module;
        struct {
            AstNode *expr;
        } expr_stmt;
        struct {
            char *name;
            AstNode *value;
        } assign;
        struct {
            AstNode *test;
            PtrVec body;
            PtrVec orelse;
        } if_stmt;
        struct {
            AstNode *test;
            PtrVec body;
        } while_stmt;
        struct {
            char *name;
            char **params;
            int param_count;
            PtrVec body;
        } function_def;
        struct {
            AstNode *value;
        } return_stmt;
        struct {
            char *name;
        } name;
        struct {
            int is_float;
            long int_value;
            double float_value;
            char *str_value;
            int is_string;
        } constant;
        struct {
            OpKind op;
            AstNode *left;
            AstNode *right;
        } binop;
        struct {
            OpKind op;
            AstNode *operand;
        } unaryop;
        struct {
            OpKind op;
            AstNode *left;
            AstNode *right;
        } compare;
        struct {
            AstNode *func;
            PtrVec args;
        } call;
    } as;
};

AstNode *ast_new_module(void);
AstNode *ast_new_expr_stmt(AstNode *expr, int line);
AstNode *ast_new_assign(char *name, AstNode *value, int line);
AstNode *ast_new_if(AstNode *test, int line);
AstNode *ast_new_while(AstNode *test, int line);
AstNode *ast_new_function_def(char *name, char **params, int param_count, int line);
AstNode *ast_new_return(AstNode *value, int line);
AstNode *ast_new_pass(int line);
AstNode *ast_new_name(char *name, int line);
AstNode *ast_new_int(long value, int line);
AstNode *ast_new_float(double value, int line);
AstNode *ast_new_string(char *value, int line);
AstNode *ast_new_binop(OpKind op, AstNode *left, AstNode *right, int line);
AstNode *ast_new_unaryop(OpKind op, AstNode *operand, int line);
AstNode *ast_new_compare(OpKind op, AstNode *left, AstNode *right, int line);
AstNode *ast_new_call(AstNode *func, int line);
void ast_free(AstNode *node);

#endif
