#include "ast.h"

#include "lexer.h"
#include "parser.h"

AstNode *parse_source(const char *source, const char *filename) {
    TokenVec tokens;
    lex_source(source, &tokens);
    AstNode *module = parse_tokens(&tokens, filename);
    free_tokens(&tokens);
    return module;
}
