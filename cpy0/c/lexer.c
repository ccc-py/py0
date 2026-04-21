#include "lexer.h"

#include <ctype.h>
#include <stdlib.h>
#include <string.h>

static void tokenvec_push(TokenVec *vec, Token tok) {
    if (vec->count == vec->capacity) {
        int new_cap = vec->capacity ? vec->capacity * 2 : 64;
        Token *new_items = (Token *)realloc(vec->items, sizeof(Token) * new_cap);
        if (!new_items) {
            die("out of memory");
        }
        vec->items = new_items;
        vec->capacity = new_cap;
    }
    vec->items[vec->count++] = tok;
}

static void emit(TokenVec *tokens, TokenKind kind, char *lexeme, int line, int col) {
    Token tok;
    tok.kind = kind;
    tok.lexeme = lexeme;
    tok.line = line;
    tok.col = col;
    tokenvec_push(tokens, tok);
}

static TokenKind keyword_kind(const char *text) {
    if (strcmp(text, "def") == 0) return TOK_DEF;
    if (strcmp(text, "if") == 0) return TOK_IF;
    if (strcmp(text, "else") == 0) return TOK_ELSE;
    if (strcmp(text, "while") == 0) return TOK_WHILE;
    if (strcmp(text, "return") == 0) return TOK_RETURN;
    if (strcmp(text, "pass") == 0) return TOK_PASS;
    return TOK_NAME;
}

static void lex_line(const char *line, int line_no, int start_col, TokenVec *tokens) {
    int i = start_col;
    while (line[i] && line[i] != '\n' && line[i] != '\r') {
        char c = line[i];
        if (c == '#') {
            break;
        }
        if (c == ' ' || c == '\t') {
            i++;
            continue;
        }
        if (isalpha((unsigned char)c) || c == '_') {
            int start = i;
            i++;
            while (isalnum((unsigned char)line[i]) || line[i] == '_') {
                i++;
            }
            char *text = xstrndup(line + start, (size_t)(i - start));
            emit(tokens, keyword_kind(text), text, line_no, start + 1);
            continue;
        }
        if (isdigit((unsigned char)c)) {
            int start = i;
            int is_float = 0;
            i++;
            while (isdigit((unsigned char)line[i])) i++;
            if (line[i] == '.') {
                is_float = 1;
                i++;
                while (isdigit((unsigned char)line[i])) i++;
            }
            char *text = xstrndup(line + start, (size_t)(i - start));
            emit(tokens, is_float ? TOK_FLOAT : TOK_INT, text, line_no, start + 1);
            continue;
        }
        if (c == '\'' || c == '"') {
            char quote = c;
            int start = i;
            StrBuf buf;
            strbuf_init(&buf);
            i++;
            while (line[i] && line[i] != quote) {
                if (line[i] == '\\') {
                    i++;
                    if (!line[i]) break;
                    switch (line[i]) {
                        case 'n': strbuf_append_char(&buf, '\n'); break;
                        case 't': strbuf_append_char(&buf, '\t'); break;
                        case '\\': strbuf_append_char(&buf, '\\'); break;
                        case '\'': strbuf_append_char(&buf, '\''); break;
                        case '"': strbuf_append_char(&buf, '"'); break;
                        default: strbuf_append_char(&buf, line[i]); break;
                    }
                    i++;
                } else {
                    strbuf_append_char(&buf, line[i]);
                    i++;
                }
            }
            if (line[i] != quote) {
                die("unterminated string at line %d", line_no);
            }
            i++;
            emit(tokens, TOK_STRING, strbuf_take(&buf), line_no, start + 1);
            continue;
        }
        if (c == '=' && line[i + 1] == '=') {
            emit(tokens, TOK_EQEQ, xstrdup("=="), line_no, i + 1);
            i += 2;
            continue;
        }
        if (c == '!' && line[i + 1] == '=') {
            emit(tokens, TOK_NE, xstrdup("!="), line_no, i + 1);
            i += 2;
            continue;
        }
        if (c == '<' && line[i + 1] == '=') {
            emit(tokens, TOK_LE, xstrdup("<="), line_no, i + 1);
            i += 2;
            continue;
        }
        if (c == '>' && line[i + 1] == '=') {
            emit(tokens, TOK_GE, xstrdup(">="), line_no, i + 1);
            i += 2;
            continue;
        }
        TokenKind kind = TOK_EOF;
        switch (c) {
            case '(': kind = TOK_LPAREN; break;
            case ')': kind = TOK_RPAREN; break;
            case '[': kind = TOK_LBRACKET; break;
            case ']': kind = TOK_RBRACKET; break;
            case ',': kind = TOK_COMMA; break;
            case ':': kind = TOK_COLON; break;
            case '.': kind = TOK_DOT; break;
            case '+': kind = TOK_PLUS; break;
            case '-': kind = TOK_MINUS; break;
            case '*': kind = TOK_STAR; break;
            case '/': kind = TOK_SLASH; break;
            case '%': kind = TOK_PERCENT; break;
            case '=': kind = TOK_EQUAL; break;
            case '<': kind = TOK_LT; break;
            case '>': kind = TOK_GT; break;
            default:
                die("unexpected character '%c' at line %d", c, line_no);
        }
        char *text = xstrndup(&line[i], 1);
        emit(tokens, kind, text, line_no, i + 1);
        i++;
    }
}

void lex_source(const char *source, TokenVec *out_tokens) {
    out_tokens->items = NULL;
    out_tokens->count = 0;
    out_tokens->capacity = 0;

    int indent_stack[128];
    int indent_top = 0;
    indent_stack[0] = 0;

    const char *cursor = source;
    int line_no = 1;
    while (*cursor) {
        const char *line_start = cursor;
        while (*cursor && *cursor != '\n') {
            cursor++;
        }
        size_t len = (size_t)(cursor - line_start);
        if (*cursor == '\n') {
            cursor++;
        }
        char *line = xstrndup(line_start, len);

        int col = 0;
        int indent = 0;
        while (line[col] == ' ' || line[col] == '\t') {
            indent += (line[col] == '\t') ? 4 : 1;
            col++;
        }

        int blank = 1;
        for (int i = col; line[i]; i++) {
            if (line[i] == '#') {
                break;
            }
            if (!isspace((unsigned char)line[i])) {
                blank = 0;
                break;
            }
        }

        if (!blank) {
            if (indent > indent_stack[indent_top]) {
                indent_stack[++indent_top] = indent;
                emit(out_tokens, TOK_INDENT, xstrdup("<INDENT>"), line_no, 1);
            } else {
                while (indent < indent_stack[indent_top]) {
                    indent_top--;
                    emit(out_tokens, TOK_DEDENT, xstrdup("<DEDENT>"), line_no, 1);
                }
                if (indent != indent_stack[indent_top]) {
                    die("inconsistent indentation at line %d", line_no);
                }
            }
            lex_line(line, line_no, col, out_tokens);
            emit(out_tokens, TOK_NEWLINE, xstrdup("<NEWLINE>"), line_no, (int)len + 1);
        }

        free(line);
        line_no++;
    }

    while (indent_top > 0) {
        indent_top--;
        emit(out_tokens, TOK_DEDENT, xstrdup("<DEDENT>"), line_no, 1);
    }
    emit(out_tokens, TOK_EOF, xstrdup("<EOF>"), line_no, 1);
}

void free_tokens(TokenVec *tokens) {
    for (int i = 0; i < tokens->count; i++) {
        free(tokens->items[i].lexeme);
    }
    free(tokens->items);
    tokens->items = NULL;
    tokens->count = 0;
    tokens->capacity = 0;
}
