use crate::ast_nodes::AstNode;
use crate::value::{PyEnv, PyFunction, PyValue};

pub fn py_make_function(
    name: String,
    params: Vec<String>,
    param_count: usize,
    body: AstNode,
    closure: *const PyEnv,
) -> PyValue {
    PyValue::new_function(PyFunction {
        name,
        params,
        param_count,
        body,
        closure,
    })
}