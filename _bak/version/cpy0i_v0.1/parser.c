#include "parser.h"

#include <stdlib.h>
#include <string.h>

typedef struct {
    TokenVec *tokens;
    int pos;
    const char *filename;
} Parser;

static Token *peek(Parser *p) {
    return &p->tokens->items[p->pos];
}

static Token *prev(Parser *p) {
    return &p->tokens->items[p->pos - 1];
}

static int match(Parser *p, TokenKind kind) {
    if (peek(p)->kind == kind) {
        p->pos++;
        return 1;
    }
    return 0;
}

static Token *expect(Parser *p, TokenKind kind, const char *message) {
    if (peek(p)->kind != kind) {
        die("%s:%d:%d: %s", p->filename, peek(p)->line, peek(p)->col, message);
    }
    p->pos++;
    return prev(p);
}

static void skip_newlines(Parser *p) {
    while (match(p, TOK_NEWLINE)) {
    }
}

static AstNode *parse_expr(Parser *p);

static AstNode *parse_primary(Parser *p) {
    Token *tok = peek(p);
    if (match(p, TOK_INT)) {
        return ast_new_int(strtol(prev(p)->lexeme, NULL, 10), tok->line);
    }
    if (match(p, TOK_FLOAT)) {
        return ast_new_float(strtod(prev(p)->lexeme, NULL), tok->line);
    }
    if (match(p, TOK_STRING)) {
        return ast_new_string(xstrdup(prev(p)->lexeme), tok->line);
    }
    if (match(p, TOK_NAME)) {
        AstNode *expr = ast_new_name(xstrdup(prev(p)->lexeme), tok->line);
        while (match(p, TOK_LPAREN)) {
            AstNode *call = ast_new_call(expr, tok->line);
            if (!match(p, TOK_RPAREN)) {
                do {
                    ptrvec_push(&call->as.call.args, parse_expr(p));
                } while (match(p, TOK_COMMA));
                expect(p, TOK_RPAREN, "expected ')'");
            }
            expr = call;
        }
        return expr;
    }
    if (match(p, TOK_LPAREN)) {
        AstNode *expr = parse_expr(p);
        expect(p, TOK_RPAREN, "expected ')'");
        return expr;
    }
    die("%s:%d:%d: expected expression", p->filename, tok->line, tok->col);
    return NULL;
}

static AstNode *parse_unary(Parser *p) {
    if (match(p, TOK_MINUS)) {
        return ast_new_unaryop(OP_NEG, parse_unary(p), prev(p)->line);
    }
    return parse_primary(p);
}

static AstNode *parse_factor(Parser *p) {
    AstNode *expr = parse_unary(p);
    for (;;) {
        OpKind op;
        if (match(p, TOK_STAR)) op = OP_MUL;
        else if (match(p, TOK_SLASH)) op = OP_DIV;
        else if (match(p, TOK_PERCENT)) op = OP_MOD;
        else break;
        expr = ast_new_binop(op, expr, parse_unary(p), prev(p)->line);
    }
    return expr;
}

static AstNode *parse_term(Parser *p) {
    AstNode *expr = parse_factor(p);
    for (;;) {
        OpKind op;
        if (match(p, TOK_PLUS)) op = OP_ADD;
        else if (match(p, TOK_MINUS)) op = OP_SUB;
        else break;
        expr = ast_new_binop(op, expr, parse_factor(p), prev(p)->line);
    }
    return expr;
}

static AstNode *parse_comparison(Parser *p) {
    AstNode *expr = parse_term(p);
    for (;;) {
        OpKind op;
        if (match(p, TOK_EQEQ)) op = OP_EQ;
        else if (match(p, TOK_NE)) op = OP_NE;
        else if (match(p, TOK_LT)) op = OP_LT;
        else if (match(p, TOK_LE)) op = OP_LE;
        else if (match(p, TOK_GT)) op = OP_GT;
        else if (match(p, TOK_GE)) op = OP_GE;
        else break;
        expr = ast_new_compare(op, expr, parse_term(p), prev(p)->line);
    }
    return expr;
}

static AstNode *parse_expr(Parser *p) {
    return parse_comparison(p);
}

static AstNode *parse_stmt(Parser *p);

static void parse_block(Parser *p, PtrVec *body) {
    expect(p, TOK_NEWLINE, "expected newline after ':'");
    expect(p, TOK_INDENT, "expected indented block");
    skip_newlines(p);
    while (peek(p)->kind != TOK_DEDENT && peek(p)->kind != TOK_EOF) {
        ptrvec_push(body, parse_stmt(p));
        skip_newlines(p);
    }
    expect(p, TOK_DEDENT, "expected dedent");
}

static AstNode *parse_function_def(Parser *p) {
    Token *name = expect(p, TOK_NAME, "expected function name");
    expect(p, TOK_LPAREN, "expected '(' after function name");
    char **params = NULL;
    int param_count = 0;
    int param_cap = 0;
    if (!match(p, TOK_RPAREN)) {
        do {
            Token *param = expect(p, TOK_NAME, "expected parameter name");
            if (param_count == param_cap) {
                int new_cap = param_cap ? param_cap * 2 : 4;
                char **new_params = (char **)realloc(params, sizeof(char *) * new_cap);
                if (!new_params) {
                    die("out of memory");
                }
                params = new_params;
                param_cap = new_cap;
            }
            params[param_count++] = xstrdup(param->lexeme);
        } while (match(p, TOK_COMMA));
        expect(p, TOK_RPAREN, "expected ')'");
    }
    expect(p, TOK_COLON, "expected ':' after function definition");
    AstNode *fn = ast_new_function_def(xstrdup(name->lexeme), params, param_count, name->line);
    parse_block(p, &fn->as.function_def.body);
    return fn;
}

static AstNode *parse_if(Parser *p) {
    Token *tok = prev(p);
    AstNode *test = parse_expr(p);
    expect(p, TOK_COLON, "expected ':' after if condition");
    AstNode *node = ast_new_if(test, tok->line);
    parse_block(p, &node->as.if_stmt.body);
    skip_newlines(p);
    if (match(p, TOK_ELSE)) {
        expect(p, TOK_COLON, "expected ':' after else");
        parse_block(p, &node->as.if_stmt.orelse);
    }
    return node;
}

static AstNode *parse_while(Parser *p) {
    Token *tok = prev(p);
    AstNode *test = parse_expr(p);
    expect(p, TOK_COLON, "expected ':' after while condition");
    AstNode *node = ast_new_while(test, tok->line);
    parse_block(p, &node->as.while_stmt.body);
    return node;
}

static AstNode *parse_stmt(Parser *p) {
    if (match(p, TOK_DEF)) {
        return parse_function_def(p);
    }
    if (match(p, TOK_IF)) {
        return parse_if(p);
    }
    if (match(p, TOK_WHILE)) {
        return parse_while(p);
    }
    if (match(p, TOK_RETURN)) {
        int line = prev(p)->line;
        if (peek(p)->kind == TOK_NEWLINE) {
            expect(p, TOK_NEWLINE, "expected newline");
            return ast_new_return(NULL, line);
        }
        AstNode *expr = parse_expr(p);
        expect(p, TOK_NEWLINE, "expected newline after return");
        return ast_new_return(expr, line);
    }
    if (match(p, TOK_PASS)) {
        int line = prev(p)->line;
        expect(p, TOK_NEWLINE, "expected newline after pass");
        return ast_new_pass(line);
    }

    if (peek(p)->kind == TOK_NAME && p->tokens->items[p->pos + 1].kind == TOK_EQUAL) {
        Token *name = expect(p, TOK_NAME, "expected name");
        expect(p, TOK_EQUAL, "expected '='");
        AstNode *value = parse_expr(p);
        expect(p, TOK_NEWLINE, "expected newline after assignment");
        return ast_new_assign(xstrdup(name->lexeme), value, name->line);
    }

    AstNode *expr = parse_expr(p);
    expect(p, TOK_NEWLINE, "expected newline after expression");
    return ast_new_expr_stmt(expr, expr->line);
}

AstNode *parse_tokens(TokenVec *tokens, const char *filename) {
    Parser p;
    p.tokens = tokens;
    p.pos = 0;
    p.filename = filename;

    AstNode *module = ast_new_module();
    skip_newlines(&p);
    while (peek(&p)->kind != TOK_EOF) {
        ptrvec_push(&module->as.module.body, parse_stmt(&p));
        skip_newlines(&p);
    }
    return module;
}
