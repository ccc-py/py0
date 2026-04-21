#ifndef CPY0I_LEXER_H
#define CPY0I_LEXER_H

#include "util.h"

typedef enum {
    TOK_EOF,
    TOK_NEWLINE,
    TOK_INDENT,
    TOK_DEDENT,
    TOK_NAME,
    TOK_INT,
    TOK_FLOAT,
    TOK_STRING,
    TOK_DEF,
    TOK_IF,
    TOK_ELSE,
    TOK_WHILE,
    TOK_RETURN,
    TOK_PASS,
    TOK_LPAREN,
    TOK_RPAREN,
    TOK_COMMA,
    TOK_COLON,
    TOK_PLUS,
    TOK_MINUS,
    TOK_STAR,
    TOK_SLASH,
    TOK_PERCENT,
    TOK_EQUAL,
    TOK_EQEQ,
    TOK_NE,
    TOK_LT,
    TOK_LE,
    TOK_GT,
    TOK_GE
} TokenKind;

typedef struct {
    TokenKind kind;
    char *lexeme;
    int line;
    int col;
} Token;

typedef struct {
    Token *items;
    int count;
    int capacity;
} TokenVec;

void lex_source(const char *source, TokenVec *out_tokens);
void free_tokens(TokenVec *tokens);

#endif
