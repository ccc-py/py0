use crate::ast_nodes::AstNode;
use crate::lexer::Lexer;

pub fn parse_source(source: &str, _filename: &str) -> AstNode {
    let mut lexer = Lexer::new(source);
    lexer.scan_tokens();
    let tokens = lexer.get_tokens();
    let mut parser = crate::parser::Parser::new(tokens);
    parser.parse()
}