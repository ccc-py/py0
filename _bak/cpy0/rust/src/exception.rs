use crate::value::PyValue;

#[derive(Clone)]
pub struct ExecResult {
    pub has_return: bool,
    pub value: Option<PyValue>,
}

impl ExecResult {
    pub fn new() -> Self {
        ExecResult {
            has_return: false,
            value: None,
        }
    }
}

impl Default for ExecResult {
    fn default() -> Self {
        Self::new()
    }
}