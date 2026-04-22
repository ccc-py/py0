use crate::eval::exec_function_body;
use crate::value::{PyEnv, PyRuntime, PyValue};

pub fn py_call(rt: &mut PyRuntime, callable: &PyValue, argc: usize, argv: &mut [PyValue]) -> PyValue {
    match callable.py_type {
        crate::value::PyType::BuiltinFunction => {
            let fn_ = callable.as_cfunc.as_ref().unwrap();
            (fn_.fn_)(rt, argc, argv)
        }
        crate::value::PyType::Function => {
            let fn_ = callable.as_func.as_ref().unwrap();
            if argc != fn_.param_count {
                crate::util::die(&format!(
                    "{}() expected {} arguments, got {}",
                    fn_.name, fn_.param_count, argc
                ));
            }

            let closure_ptr = fn_.closure;
            let parent_env = unsafe { &*closure_ptr };
            let mut local = PyEnv::env_new(Some(parent_env.clone()));
            for i in 0..argc {
                local.set(&fn_.params[i], argv[i].clone());
            }

            let result = exec_function_body(rt, &mut local, &fn_.body);
            if result.has_return {
                if let Some(v) = result.value {
                    return v;
                }
            }
            PyValue::new_none()
        }
        _ => {
            crate::util::die("object is not callable");
            PyValue::new_none()
        }
    }
}