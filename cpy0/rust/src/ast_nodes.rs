use crate::util::PtrVec;

#[derive(Clone, PartialEq, Debug)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Neg,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AstKind {
    Module,
    ExprStmt,
    Assign,
    If,
    While,
    FunctionDef,
    Return,
    Pass,
    Name,
    Constant,
    BinOp,
    UnaryOp,
    Compare,
    Call,
    Attribute,
    Subscript,
}

#[derive(Clone)]
pub struct AstNode {
    pub kind: AstKind,
    pub line: usize,
    pub as_module: Option<ModuleData>,
    pub as_expr_stmt: Option<ExprStmtData>,
    pub as_assign: Option<AssignData>,
    pub as_if_stmt: Option<IfStmtData>,
    pub as_while_stmt: Option<WhileStmtData>,
    pub as_function_def: Option<FunctionDefData>,
    pub as_return_stmt: Option<ReturnStmtData>,
    pub as_name: Option<NameData>,
    pub as_constant: Option<ConstantData>,
    pub as_binop: Option<BinOpData>,
    pub as_unaryop: Option<UnaryOpData>,
    pub as_compare: Option<CompareData>,
    pub as_call: Option<CallData>,
    pub as_attribute: Option<AttributeData>,
    pub as_subscript: Option<SubscriptData>,
}

#[derive(Clone)]
pub struct ModuleData {
    pub body: PtrVec<AstNode>,
}

#[derive(Clone)]
pub struct ExprStmtData {
    pub expr: Box<AstNode>,
}

#[derive(Clone)]
pub struct AssignData {
    pub name: String,
    pub value: Box<AstNode>,
}

#[derive(Clone)]
pub struct IfStmtData {
    pub test: Box<AstNode>,
    pub body: PtrVec<AstNode>,
    pub orelse: PtrVec<AstNode>,
}

#[derive(Clone)]
pub struct WhileStmtData {
    pub test: Box<AstNode>,
    pub body: PtrVec<AstNode>,
}

#[derive(Clone)]
pub struct FunctionDefData {
    pub name: String,
    pub params: Vec<String>,
    pub param_count: usize,
    pub body: PtrVec<AstNode>,
}

#[derive(Clone)]
pub struct ReturnStmtData {
    pub value: Option<Box<AstNode>>,
}

#[derive(Clone)]
pub struct NameData {
    pub name: String,
}

#[derive(Clone)]
pub struct ConstantData {
    pub is_float: bool,
    pub int_value: i64,
    pub float_value: f64,
    pub str_value: Option<String>,
    pub is_string: bool,
}

#[derive(Clone)]
pub struct BinOpData {
    pub op: OpKind,
    pub left: Box<AstNode>,
    pub right: Box<AstNode>,
}

#[derive(Clone)]
pub struct UnaryOpData {
    pub op: OpKind,
    pub operand: Box<AstNode>,
}

#[derive(Clone)]
pub struct CompareData {
    pub op: OpKind,
    pub left: Box<AstNode>,
    pub right: Box<AstNode>,
}

#[derive(Clone)]
pub struct CallData {
    pub func: Box<AstNode>,
    pub args: PtrVec<AstNode>,
}

#[derive(Clone)]
pub struct AttributeData {
    pub value: Box<AstNode>,
    pub attr: String,
}

#[derive(Clone)]
pub struct SubscriptData {
    pub value: Box<AstNode>,
    pub index: Box<AstNode>,
}

impl AstNode {
    pub fn new_module() -> Self {
        AstNode {
            kind: AstKind::Module,
            line: 1,
            as_module: Some(ModuleData {
                body: PtrVec::new(),
            }),
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_expr_stmt(expr: Box<AstNode>, line: usize) -> Self {
        AstNode {
            kind: AstKind::ExprStmt,
            line,
            as_module: None,
            as_expr_stmt: Some(ExprStmtData { expr }),
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_assign(name: String, value: Box<AstNode>, line: usize) -> Self {
        AstNode {
            kind: AstKind::Assign,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: Some(AssignData { name, value }),
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_if(test: Box<AstNode>, line: usize) -> Self {
        AstNode {
            kind: AstKind::If,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: Some(IfStmtData {
                test,
                body: PtrVec::new(),
                orelse: PtrVec::new(),
            }),
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_while(test: Box<AstNode>, line: usize) -> Self {
        AstNode {
            kind: AstKind::While,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: Some(WhileStmtData {
                test,
                body: PtrVec::new(),
            }),
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_function_def(name: String, params: Vec<String>, line: usize) -> Self {
        let param_count = params.len();
        AstNode {
            kind: AstKind::FunctionDef,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: Some(FunctionDefData {
                name,
                params,
                param_count,
                body: PtrVec::new(),
            }),
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_return(value: Option<Box<AstNode>>, line: usize) -> Self {
        AstNode {
            kind: AstKind::Return,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: Some(ReturnStmtData { value }),
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_pass(line: usize) -> Self {
        AstNode {
            kind: AstKind::Pass,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_name(name: String, line: usize) -> Self {
        AstNode {
            kind: AstKind::Name,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: Some(NameData { name }),
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_int(value: i64, line: usize) -> Self {
        AstNode {
            kind: AstKind::Constant,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: Some(ConstantData {
                is_float: false,
                int_value: value,
                float_value: 0.0,
                str_value: None,
                is_string: false,
            }),
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_float(value: f64, line: usize) -> Self {
        AstNode {
            kind: AstKind::Constant,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: Some(ConstantData {
                is_float: true,
                int_value: 0,
                float_value: value,
                str_value: None,
                is_string: false,
            }),
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_string(value: String, line: usize) -> Self {
        AstNode {
            kind: AstKind::Constant,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: Some(ConstantData {
                is_float: false,
                int_value: 0,
                float_value: 0.0,
                str_value: Some(value),
                is_string: true,
            }),
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_binop(op: OpKind, left: Box<AstNode>, right: Box<AstNode>, line: usize) -> Self {
        AstNode {
            kind: AstKind::BinOp,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: Some(BinOpData { op, left, right }),
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_unaryop(op: OpKind, operand: Box<AstNode>, line: usize) -> Self {
        AstNode {
            kind: AstKind::UnaryOp,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: Some(UnaryOpData { op, operand }),
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_compare(op: OpKind, left: Box<AstNode>, right: Box<AstNode>, line: usize) -> Self {
        AstNode {
            kind: AstKind::Compare,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: Some(CompareData { op, left, right }),
            as_call: None,
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_call(func: Box<AstNode>, line: usize) -> Self {
        AstNode {
            kind: AstKind::Call,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: Some(CallData {
                func,
                args: PtrVec::new(),
            }),
            as_attribute: None,
            as_subscript: None,
        }
    }

    pub fn new_attribute(value: Box<AstNode>, attr: String, line: usize) -> Self {
        AstNode {
            kind: AstKind::Attribute,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: Some(AttributeData { value, attr }),
            as_subscript: None,
        }
    }

    pub fn new_subscript(value: Box<AstNode>, index: Box<AstNode>, line: usize) -> Self {
        AstNode {
            kind: AstKind::Subscript,
            line,
            as_module: None,
            as_expr_stmt: None,
            as_assign: None,
            as_if_stmt: None,
            as_while_stmt: None,
            as_function_def: None,
            as_return_stmt: None,
            as_name: None,
            as_constant: None,
            as_binop: None,
            as_unaryop: None,
            as_compare: None,
            as_call: None,
            as_attribute: None,
            as_subscript: Some(SubscriptData { value, index }),
        }
    }
}