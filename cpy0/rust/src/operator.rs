use crate::ast_nodes::OpKind;
use crate::value::{py_as_number, PyValue};

pub fn py_apply_binop(op: OpKind, left: &PyValue, right: &PyValue) -> PyValue {
    if op == OpKind::Add {
        if let (Some(ref ls), Some(ref rs)) = (&left.as_str, &right.as_str) {
            let mut buf = String::new();
            buf.push_str(ls);
            buf.push_str(rs);
            return PyValue::new_string(buf);
        }
    }

    let both_ints = left.py_type == crate::value::PyType::Int && right.py_type == crate::value::PyType::Int;

    if both_ints {
        let a = left.as_int;
        let b = right.as_int;
        match op {
            OpKind::Add => return PyValue::new_int(a + b),
            OpKind::Sub => return PyValue::new_int(a - b),
            OpKind::Mul => return PyValue::new_int(a * b),
            OpKind::Div => return PyValue::new_float(a as f64 / b as f64),
            OpKind::Mod => return PyValue::new_int(a % b),
            _ => {}
        }
    }

    let a = py_as_number(left);
    let b = py_as_number(right);

    match op {
        OpKind::Add => PyValue::new_float(a + b),
        OpKind::Sub => PyValue::new_float(a - b),
        OpKind::Mul => PyValue::new_float(a * b),
        OpKind::Div => PyValue::new_float(a / b),
        OpKind::Mod => PyValue::new_float((a as i64 % b as i64) as f64),
        _ => {
            crate::util::die("unsupported binary operator");
            PyValue::new_none()
        }
    }
}

pub fn py_apply_compare(op: OpKind, left: &PyValue, right: &PyValue) -> PyValue {
    if let (Some(ref ls), Some(ref rs)) = (&left.as_str, &right.as_str) {
        let cmp = ls.cmp(rs);
        match op {
            OpKind::Eq => return PyValue::new_bool(cmp == std::cmp::Ordering::Equal),
            OpKind::Ne => return PyValue::new_bool(cmp != std::cmp::Ordering::Equal),
            OpKind::Lt => return PyValue::new_bool(cmp == std::cmp::Ordering::Less),
            OpKind::Le => return PyValue::new_bool(cmp == std::cmp::Ordering::Less || cmp == std::cmp::Ordering::Equal),
            OpKind::Gt => return PyValue::new_bool(cmp == std::cmp::Ordering::Greater),
            OpKind::Ge => return PyValue::new_bool(cmp == std::cmp::Ordering::Greater || cmp == std::cmp::Ordering::Equal),
            _ => {}
        }
    }

    let a = py_as_number(left);
    let b = py_as_number(right);

    match op {
        OpKind::Eq => PyValue::new_bool((a - b).abs() < f64::EPSILON),
        OpKind::Ne => PyValue::new_bool((a - b).abs() >= f64::EPSILON),
        OpKind::Lt => PyValue::new_bool(a < b),
        OpKind::Le => PyValue::new_bool(a <= b),
        OpKind::Gt => PyValue::new_bool(a > b),
        OpKind::Ge => PyValue::new_bool(a >= b),
        _ => {
            crate::util::die("unsupported comparison operator");
            PyValue::new_bool(false)
        }
    }
}

pub fn py_apply_unary(op: OpKind, value: &PyValue) -> PyValue {
    match op {
        OpKind::Neg => {
            if value.py_type == crate::value::PyType::Int {
                return PyValue::new_int(-value.as_int);
            }
            PyValue::new_float(-py_as_number(value))
        }
        _ => {
            crate::util::die("unsupported unary operator");
            PyValue::new_none()
        }
    }
}