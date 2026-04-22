use crate::ast_nodes::{AstKind, AstNode, OpKind};
use crate::call::py_call;
use crate::exception::ExecResult;
use crate::function::py_make_function;
use crate::operator::{py_apply_binop, py_apply_compare, py_apply_unary};
use crate::value::py_is_truthy;
use crate::value::PyEnv;
use crate::value::PyRuntime;
use crate::value::PyValue;

pub fn eval_expr(rt: &mut PyRuntime, env: &mut PyEnv, node: &AstNode) -> PyValue {
    match &node.kind {
        AstKind::Name => {
            let name = node.as_name.as_ref().unwrap();
            env.env_get(&name.name)
        }
        AstKind::Constant => {
            let c = node.as_constant.as_ref().unwrap();
            if c.is_string {
                PyValue::new_string(c.str_value.as_ref().unwrap().clone())
            } else if c.is_float {
                PyValue::new_float(c.float_value)
            } else {
                PyValue::new_int(c.int_value)
            }
        }
        AstKind::BinOp => {
            let b = node.as_binop.as_ref().unwrap();
            let left = eval_expr(rt, env, &b.left);
            let right = eval_expr(rt, env, &b.right);
            py_apply_binop(b.op.clone(), &left, &right)
        }
        AstKind::UnaryOp => {
            let u = node.as_unaryop.as_ref().unwrap();
            let value = eval_expr(rt, env, &u.operand);
            py_apply_unary(u.op.clone(), &value)
        }
        AstKind::Compare => {
            let c = node.as_compare.as_ref().unwrap();
            let left = eval_expr(rt, env, &c.left);
            let right = eval_expr(rt, env, &c.right);
            py_apply_compare(c.op.clone(), &left, &right)
        }
        AstKind::Call => {
            let c = node.as_call.as_ref().unwrap();
            let func = eval_expr(rt, env, &c.func);
            let mut args = Vec::new();
            for arg in c.args.iter() {
                args.push(eval_expr(rt, env, arg));
            }
            let mut args_slice = args.as_mut_slice();
            py_call(rt, &func, args_slice.len(), &mut args_slice)
        }
        AstKind::Attribute => {
            let a = node.as_attribute.as_ref().unwrap();
            let value = eval_expr(rt, env, &a.value);
            if value.py_type == crate::value::PyType::Sys && a.attr == "argv" {
                if let Some(ref sys) = value.as_sys {
                    return sys.argv.clone();
                }
            }
            crate::util::die(&format!("unsupported attribute access: {}", a.attr));
            PyValue::new_none()
        }
        AstKind::Subscript => {
            let s = node.as_subscript.as_ref().unwrap();
            let value = eval_expr(rt, env, &s.value);
            let index = eval_expr(rt, env, &s.index);
            if index.py_type != crate::value::PyType::Int {
                crate::util::die("subscript index must be int");
            }
            match value.py_type {
                crate::value::PyType::List => {
                    if let Some(ref l) = value.as_list {
                        let i = index.as_int as usize;
                        if i >= l.items.len() {
                            crate::util::die("list index out of range");
                        }
                        return l.items[i].clone();
                    }
                }
                crate::value::PyType::Str => {
                    if let Some(ref s_str) = value.as_str {
                        let i = index.as_int as usize;
                        if i >= s_str.len() {
                            crate::util::die("string index out of range");
                        }
                        let ch = s_str.chars().nth(i).unwrap();
                        return PyValue::new_string(ch.to_string());
                    }
                }
                _ => {}
            }
            crate::util::die("unsupported subscript target");
            PyValue::new_none()
        }
        _ => {
            crate::util::die(&format!("unsupported expression kind {:?}", node.kind));
            PyValue::new_none()
        }
    }
}

fn exec_block(rt: &mut PyRuntime, env: &mut PyEnv, body: &[AstNode]) -> ExecResult {
    let mut result = ExecResult::new();
    for node in body {
        result = exec_stmt(rt, env, node);
        if result.has_return {
            return result;
        }
    }
    result
}

pub fn exec_stmt(rt: &mut PyRuntime, env: &mut PyEnv, node: &AstNode) -> ExecResult {
    let result = ExecResult::new();
    match &node.kind {
        AstKind::ExprStmt => {
            let e = node.as_expr_stmt.as_ref().unwrap();
            eval_expr(rt, env, e.expr.as_ref());
            result
        }
        AstKind::Assign => {
            let a = node.as_assign.as_ref().unwrap();
            let value = eval_expr(rt, env, a.value.as_ref());
            env.env_assign(&a.name, value);
            result
        }
        AstKind::If => {
            let i = node.as_if_stmt.as_ref().unwrap();
            let cond = eval_expr(rt, env, i.test.as_ref());
            if py_is_truthy(&cond) {
                exec_block(rt, env, i.body.as_slice())
            } else {
                exec_block(rt, env, i.orelse.as_slice())
            }
        }
        AstKind::While => {
            let w = node.as_while_stmt.as_ref().unwrap();
            let mut result = ExecResult::new();
            while py_is_truthy(&eval_expr(rt, env, w.test.as_ref())) {
                result = exec_block(rt, env, w.body.as_slice());
                if result.has_return {
                    return result;
                }
            }
            result
        }
        AstKind::FunctionDef => {
            let f = node.as_function_def.as_ref().unwrap();
            let func_body = node.clone();
            let fn_value_without_closure = py_make_function(
                f.name.clone(),
                f.params.clone(),
                f.param_count,
                func_body.clone(),
                env as *const PyEnv,
            );
            env.env_set(&f.name, fn_value_without_closure);
            let fn_value_with_closure = py_make_function(
                f.name.clone(),
                f.params.clone(),
                f.param_count,
                func_body,
                env as *const PyEnv,
            );
            env.env_set(&f.name, fn_value_with_closure);
            result
        }
        AstKind::Return => {
            let r = node.as_return_stmt.as_ref().unwrap();
            let mut result = ExecResult::new();
            result.has_return = true;
            result.value = if let Some(ref v) = r.value {
                Some(eval_expr(rt, env, v.as_ref()))
            } else {
                Some(PyValue::new_none())
            };
            result
        }
        AstKind::Pass => result,
        _ => {
            crate::util::die(&format!("unsupported statement kind {:?}", node.kind));
            result
        }
    }
}

pub fn exec_module(rt: &mut PyRuntime, env: &mut PyEnv, module: &AstNode) -> ExecResult {
    if module.kind != AstKind::Module {
        crate::util::die("expected module");
    }
    let m = module.as_module.as_ref().unwrap();
    exec_block(rt, env, m.body.as_slice())
}

pub fn exec_function_body(rt: &mut PyRuntime, env: &mut PyEnv, function_def: &AstNode) -> ExecResult {
    if function_def.kind != AstKind::FunctionDef {
        crate::util::die("expected function definition");
    }
    let f = function_def.as_function_def.as_ref().unwrap();
    exec_block(rt, env, f.body.as_slice())
}