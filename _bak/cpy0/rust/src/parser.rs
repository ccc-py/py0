use crate::ast_nodes::{AstKind, AstNode, OpKind};
use crate::lexer::{Token, TokenKind};
use crate::util::PtrVec;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.tokens[self.current].kind == TokenKind::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().kind == kind
        }
    }

    fn match_kinds(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, kind: &TokenKind, _message: &str) -> &Token {
        if self.check(kind) {
            return self.advance();
        }
        crate::util::die("expected token");
        self.previous()
    }

    fn parse_module(&mut self) -> AstNode {
        let mut module = AstNode::new_module();
        if let Some(ref mut m) = module.as_module {
            while !self.is_at_end() && self.check(&TokenKind::Newline) {
                self.advance();
            }
            while !self.is_at_end() && self.peek().kind != TokenKind::Eof {
                let stmt = self.parse_statement();
                m.body.push(stmt);
                while !self.is_at_end() && self.check(&TokenKind::Newline) {
                    self.advance();
                }
            }
        }
        module
    }

    fn parse_statement(&mut self) -> AstNode {
        if self.match_kinds(&[TokenKind::Def]) {
            return self.parse_function_def();
        }
        if self.match_kinds(&[TokenKind::If]) {
            return self.parse_if_stmt();
        }
        if self.match_kinds(&[TokenKind::While]) {
            return self.parse_while_stmt();
        }
        if self.match_kinds(&[TokenKind::Return]) {
            return self.parse_return_stmt();
        }
        if self.match_kinds(&[TokenKind::Pass]) {
            return self.parse_pass_stmt();
        }
        self.parse_expr_or_assign()
    }

    fn parse_function_def(&mut self) -> AstNode {
        let line = self.previous().line;
        let name = self.consume(&TokenKind::Name, "expected function name").lexeme.clone();
        self.consume(&TokenKind::Lparen, "expected '(' after function name");
        let params = self.parse_param_list();
        self.consume(&TokenKind::Rparen, "expected ')' after parameters");
        self.consume(&TokenKind::Colon, "expected ':' after function definition");
        let mut node = AstNode::new_function_def(name, params, line);
        self.parse_block(&mut node);
        node
    }

    fn parse_param_list(&mut self) -> Vec<String> {
        let mut params = Vec::new();
        if !self.check(&TokenKind::Rparen) {
            params.push(self.consume(&TokenKind::Name, "expected parameter name").lexeme.clone());
            while self.match_kinds(&[TokenKind::Comma]) {
                params.push(self.consume(&TokenKind::Name, "expected parameter name").lexeme.clone());
            }
        }
        params
    }

    fn parse_if_stmt(&mut self) -> AstNode {
        let line = self.previous().line;
        let test = self.parse_expression();
        self.consume(&TokenKind::Colon, "expected ':' after if condition");
        let mut node = AstNode::new_if(test, line);
        self.parse_block(&mut node);
        if self.match_kinds(&[TokenKind::Else]) {
            self.consume(&TokenKind::Colon, "expected ':' after else");
            if let Some(ref mut i) = node.as_if_stmt {
                self.parse_dedent_block(&mut i.orelse);
            }
        }
        node
    }

    fn parse_while_stmt(&mut self) -> AstNode {
        let line = self.previous().line;
        let test = self.parse_expression();
        self.consume(&TokenKind::Colon, "expected ':' after while condition");
        let mut node = AstNode::new_while(test, line);
        self.parse_block(&mut node);
        node
    }

    fn parse_return_stmt(&mut self) -> AstNode {
        let line = self.previous().line;
        let value = if self.check(&TokenKind::Newline) || self.is_at_end() {
            None
        } else {
            Some(self.parse_expression())
        };
        AstNode::new_return(value, line)
    }

    fn parse_pass_stmt(&mut self) -> AstNode {
        let line = self.previous().line;
        AstNode::new_pass(line)
    }

    fn parse_expr_or_assign(&mut self) -> AstNode {
        let expr = self.parse_expression();
        if self.match_kinds(&[TokenKind::Equal]) {
            if let AstKind::Name = expr.kind {
                let name = expr.as_name.as_ref().unwrap().name.clone();
                let value = self.parse_expression();
                return AstNode::new_assign(name, value, expr.line);
            }
            crate::util::die("invalid assignment target");
        }
        let line = expr.line;
        AstNode::new_expr_stmt(expr, line)
    }

    fn parse_block(&mut self, node: &mut AstNode) {
        self.consume(&TokenKind::Newline, "expected newline after ':'");
        self.consume(&TokenKind::Indent, "expected indentation");
        match node.kind {
            AstKind::If => {
                if let Some(ref mut i) = node.as_if_stmt {
                    self.parse_dedent_block(&mut i.body);
                }
            }
            AstKind::While => {
                if let Some(ref mut w) = node.as_while_stmt {
                    self.parse_dedent_block(&mut w.body);
                }
            }
            AstKind::FunctionDef => {
                if let Some(ref mut f) = node.as_function_def {
                    self.parse_dedent_block(&mut f.body);
                }
            }
            _ => {}
        }
        self.consume(&TokenKind::Dedent, "expected dedent");
    }

    fn parse_dedent_block(&mut self, body: &mut PtrVec<AstNode>) {
        while !self.check(&TokenKind::Dedent) && !self.is_at_end() {
            while !self.is_at_end() && self.check(&TokenKind::Newline) {
                self.advance();
            }
            if self.check(&TokenKind::Dedent) || self.is_at_end() {
                break;
            }
            let stmt = self.parse_statement();
            body.push(stmt);
            while !self.is_at_end() && self.check(&TokenKind::Newline) {
                self.advance();
            }
        }
    }

    fn parse_expression(&mut self) -> Box<AstNode> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Box<AstNode> {
        let mut left = self.parse_term();
        while self.match_kinds(&[TokenKind::EqEq, TokenKind::Ne, TokenKind::Lt, TokenKind::Le, TokenKind::Gt, TokenKind::Ge]) {
            let op = self.previous_operator();
            let right = self.parse_term();
            let line = self.previous().line;
            left = Box::new(AstNode::new_compare(op, left, right, line));
        }
        left
    }

    fn parse_term(&mut self) -> Box<AstNode> {
        let mut left = self.parse_factor();
        while self.match_kinds(&[TokenKind::Plus, TokenKind::Minus]) {
            let op = self.previous_operator();
            let right = self.parse_factor();
            let line = self.previous().line;
            left = Box::new(AstNode::new_binop(op, left, right, line));
        }
        left
    }

    fn parse_factor(&mut self) -> Box<AstNode> {
        let mut left = self.parse_unary();
        while self.match_kinds(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent]) {
            let op = self.previous_operator();
            let right = self.parse_unary();
            let line = self.previous().line;
            left = Box::new(AstNode::new_binop(op, left, right, line));
        }
        left
    }

    fn parse_unary(&mut self) -> Box<AstNode> {
        if self.match_kinds(&[TokenKind::Minus]) {
            let operand = self.parse_unary();
            let line = self.previous().line;
            return Box::new(AstNode::new_unaryop(OpKind::Neg, operand, line));
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Box<AstNode> {
        let mut expr = self.parse_primary();
        loop {
            if self.match_kinds(&[TokenKind::Lparen]) {
                expr = self.finish_call(*expr);
            } else if self.match_kinds(&[TokenKind::Dot]) {
                let attr = self.consume(&TokenKind::Name, "expected attribute name").lexeme.clone();
                let line = self.previous().line;
                expr = Box::new(AstNode::new_attribute(expr, attr, line));
            } else if self.match_kinds(&[TokenKind::Lbracket]) {
                let index = self.parse_expression();
                self.consume(&TokenKind::Rbracket, "expected ']' after index");
                let line = self.previous().line;
                expr = Box::new(AstNode::new_subscript(expr, index, line));
            } else {
                break;
            }
        }
        expr
    }

    fn finish_call(&mut self, func: AstNode) -> Box<AstNode> {
        let line = self.previous().line;
        let mut node = AstNode::new_call(Box::new(func), line);
        if !self.check(&TokenKind::Rparen) {
            if let Some(ref mut c) = node.as_call {
                c.args.push(*self.parse_expression());
                while self.match_kinds(&[TokenKind::Comma]) {
                    c.args.push(*self.parse_expression());
                }
            }
        }
        self.consume(&TokenKind::Rparen, "expected ')' after arguments");
        Box::new(node)
    }

    fn parse_primary(&mut self) -> Box<AstNode> {
        if self.match_kinds(&[TokenKind::Name]) {
            let name = self.previous().lexeme.clone();
            let line = self.previous().line;
            return Box::new(AstNode::new_name(name, line));
        }
        if self.match_kinds(&[TokenKind::Int]) {
            let value: i64 = self.previous().lexeme.parse().unwrap_or(0);
            let line = self.previous().line;
            return Box::new(AstNode::new_int(value, line));
        }
        if self.match_kinds(&[TokenKind::Float]) {
            let value: f64 = self.previous().lexeme.parse().unwrap_or(0.0);
            let line = self.previous().line;
            return Box::new(AstNode::new_float(value, line));
        }
        if self.match_kinds(&[TokenKind::String]) {
            let value = self.previous().lexeme.clone();
            let line = self.previous().line;
            return Box::new(AstNode::new_string(value, line));
        }
        if self.match_kinds(&[TokenKind::Lparen]) {
            let expr = self.parse_expression();
            self.consume(&TokenKind::Rparen, "expected ')' after expression");
            return expr;
        }
        crate::util::die("expected expression");
        Box::new(AstNode::new_pass(0))
    }

    fn previous_operator(&self) -> OpKind {
        match self.previous().kind {
            TokenKind::Plus => OpKind::Add,
            TokenKind::Minus => OpKind::Sub,
            TokenKind::Star => OpKind::Mul,
            TokenKind::Slash => OpKind::Div,
            TokenKind::Percent => OpKind::Mod,
            TokenKind::EqEq => OpKind::Eq,
            TokenKind::Ne => OpKind::Ne,
            TokenKind::Lt => OpKind::Lt,
            TokenKind::Le => OpKind::Le,
            TokenKind::Gt => OpKind::Gt,
            TokenKind::Ge => OpKind::Ge,
            _ => OpKind::Add,
        }
    }

    pub fn parse(&mut self) -> AstNode {
        self.parse_module()
    }
}