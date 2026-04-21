#ifndef CPY0I_PARSER_H
#define CPY0I_PARSER_H

#include "ast_nodes.h"
#include "lexer.h"

AstNode *parse_tokens(TokenVec *tokens, const char *filename);

#endif
