use crate::ast_nodes::AstNode;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum PyType {
    None,
    Bool,
    Int,
    Float,
    Str,
    List,
    Sys,
    Function,
    BuiltinFunction,
}

pub type PyCFunction = fn(&mut PyRuntime, usize, &mut [PyValue]) -> PyValue;

#[derive(Clone)]
pub struct PyFunction {
    pub name: String,
    pub params: Vec<String>,
    pub param_count: usize,
    pub body: AstNode,
    pub closure: *const PyEnv,
}

#[derive(Clone)]
pub struct PyBuiltinFunction {
    pub name: String,
    pub fn_: PyCFunction,
}

#[derive(Clone)]
pub struct PyList {
    pub items: Vec<PyValue>,
}

#[derive(Clone)]
pub struct PySys {
    pub argv: PyValue,
}

#[derive(Clone)]
pub struct PyValue {
    pub py_type: PyType,
    pub as_bool: bool,
    pub as_int: i64,
    pub as_float: f64,
    pub as_str: Option<String>,
    pub as_list: Option<Box<PyList>>,
    pub as_sys: Option<Box<PySys>>,
    pub as_func: Option<Box<PyFunction>>,
    pub as_cfunc: Option<Box<PyBuiltinFunction>>,
}

impl PyValue {
    pub fn new_none() -> Self {
        PyValue {
            py_type: PyType::None,
            as_bool: false,
            as_int: 0,
            as_float: 0.0,
            as_str: None,
            as_list: None,
            as_sys: None,
            as_func: None,
            as_cfunc: None,
        }
    }

    pub fn new_bool(value: bool) -> Self {
        PyValue {
            py_type: PyType::Bool,
            as_bool: value,
            as_int: 0,
            as_float: 0.0,
            as_str: None,
            as_list: None,
            as_sys: None,
            as_func: None,
            as_cfunc: None,
        }
    }

    pub fn new_int(value: i64) -> Self {
        PyValue {
            py_type: PyType::Int,
            as_bool: false,
            as_int: value,
            as_float: 0.0,
            as_str: None,
            as_list: None,
            as_sys: None,
            as_func: None,
            as_cfunc: None,
        }
    }

    pub fn new_float(value: f64) -> Self {
        PyValue {
            py_type: PyType::Float,
            as_bool: false,
            as_int: 0,
            as_float: value,
            as_str: None,
            as_list: None,
            as_sys: None,
            as_func: None,
            as_cfunc: None,
        }
    }

    pub fn new_string(value: String) -> Self {
        PyValue {
            py_type: PyType::Str,
            as_bool: false,
            as_int: 0,
            as_float: 0.0,
            as_str: Some(value),
            as_list: None,
            as_sys: None,
            as_func: None,
            as_cfunc: None,
        }
    }

    pub fn new_list() -> Self {
        PyValue {
            py_type: PyType::List,
            as_bool: false,
            as_int: 0,
            as_float: 0.0,
            as_str: None,
            as_list: Some(Box::new(PyList { items: Vec::new() })),
            as_sys: None,
            as_func: None,
            as_cfunc: None,
        }
    }

    pub fn new_sys(argv_list: PyValue) -> Self {
        PyValue {
            py_type: PyType::Sys,
            as_bool: false,
            as_int: 0,
            as_float: 0.0,
            as_str: None,
            as_list: None,
            as_sys: Some(Box::new(PySys { argv: argv_list })),
            as_func: None,
            as_cfunc: None,
        }
    }

    pub fn new_function(func: PyFunction) -> Self {
        PyValue {
            py_type: PyType::Function,
            as_bool: false,
            as_int: 0,
            as_float: 0.0,
            as_str: None,
            as_list: None,
            as_sys: None,
            as_func: Some(Box::new(func)),
            as_cfunc: None,
        }
    }

    pub fn new_builtin(name: String, fn_: PyCFunction) -> Self {
        PyValue {
            py_type: PyType::BuiltinFunction,
            as_bool: false,
            as_int: 0,
            as_float: 0.0,
            as_str: None,
            as_list: None,
            as_sys: None,
            as_func: None,
            as_cfunc: Some(Box::new(PyBuiltinFunction { name, fn_ })),
        }
    }
}

#[derive(Clone)]
pub struct PyEnv {
    pub parent: Option<Box<PyEnv>>,
    pub items: Vec<(String, PyValue)>,
}

impl PyEnv {
    pub fn new(parent: Option<PyEnv>) -> Self {
        PyEnv {
            parent: parent.map(Box::new),
            items: Vec::new(),
        }
    }

    pub fn env_new(parent: Option<PyEnv>) -> Self {
        PyEnv::new(parent)
    }

    pub fn set(&mut self, name: &str, value: PyValue) {
        for (i, (n, _)) in self.items.iter_mut().enumerate() {
            if *n == name {
                self.items[i].1 = value;
                return;
            }
        }
        self.items.push((name.to_string(), value));
    }

    pub fn env_set(&mut self, name: &str, value: PyValue) {
        PyEnv::set(self, name, value);
    }

    pub fn env_assign(&mut self, name: &str, value: PyValue) {
        let mut current: &mut PyEnv = self;
        loop {
            for (i, n) in current.items.iter_mut().enumerate() {
                if n.0 == name {
                    current.items[i].1 = value.clone();
                    return;
                }
            }
            match &mut current.parent {
                Some(parent) => current = parent,
                None => break,
            }
        }
        PyEnv::set(self, name, value);
    }

    pub fn env_get(&self, name: &str) -> PyValue {
        let mut current: &PyEnv = self;
        loop {
            for (n, v) in &current.items {
                if *n == name {
                    return v.clone();
                }
            }
            match &current.parent {
                Some(parent) => current = parent,
                None => break,
            }
        }
        crate::util::die(&format!("name '{}' is not defined", name));
        PyValue::new_none()
    }

    fn iter(&self) -> EnvIter {
        EnvIter {
            env: Some(self),
            visited: vec![],
        }
    }

    fn iter_mut(&mut self) -> EnvIterMut {
        EnvIterMut { env: Some(self) }
    }
}

struct EnvIter<'a> {
    env: Option<&'a PyEnv>,
    visited: Vec<*const PyEnv>,
}

impl<'a> Iterator for EnvIter<'a> {
    type Item = &'a [(String, PyValue)];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.env.take() {
            self.visited.push(e as *const PyEnv);
            Some(&e.items)
        } else {
            None
        }
    }
}

struct EnvIterMut<'a> {
    env: Option<&'a mut PyEnv>,
}

impl<'a> Iterator for EnvIterMut<'a> {
    type Item = &'a mut PyEnv;

    fn next(&mut self) -> Option<Self::Item> {
        self.env.take()
    }
}

pub struct PyRuntime {
    pub globals: PyEnv,
    pub sys_value: PyValue,
}

impl PyRuntime {
    pub fn new() -> Self {
        let mut rt = PyRuntime {
            globals: PyEnv::new(None),
            sys_value: PyValue::new_none(),
        };
        PyRuntime::init_builtins(&mut rt);
        rt
    }

    pub fn init_builtins(&mut self) {
        self.globals.set("print", PyValue::new_builtin("print".to_string(), builtin_print));
        self.globals.set("len", PyValue::new_builtin("len".to_string(), builtin_len));
        self.globals.set("int", PyValue::new_builtin("int".to_string(), builtin_int));
        self.globals.set("float", PyValue::new_builtin("float".to_string(), builtin_float));
        self.globals.set("str", PyValue::new_builtin("str".to_string(), builtin_str));
        self.globals.set("bool", PyValue::new_builtin("bool".to_string(), builtin_bool));
        self.globals.set("__import__", PyValue::new_builtin("__import__".to_string(), builtin_import));
        self.globals.set("run_path", PyValue::new_builtin("run_path".to_string(), builtin_run_path));
    }

    pub fn set_argv(&mut self, _argc: usize, argv: &[&str]) {
        let mut list = PyValue::new_list();
        if let Some(ref mut l) = list.as_list {
            for arg in argv {
                l.items.push(PyValue::new_string(arg.to_string()));
            }
        }
        self.sys_value = PyValue::new_sys(list);
        self.globals.set("sys", self.sys_value.clone());
    }
}

impl Default for PyRuntime {
    fn default() -> Self {
        Self::new()
    }
}

fn builtin_print(rt: &mut PyRuntime, argc: usize, argv: &mut [PyValue]) -> PyValue {
    let _ = rt;
    for i in 0..argc {
        let text = py_to_string(&argv[i]);
        if i > 0 {
            print!(" ");
        }
        print!("{}", text);
    }
    println!();
    PyValue::new_none()
}

fn builtin_len(rt: &mut PyRuntime, argc: usize, argv: &mut [PyValue]) -> PyValue {
    let _ = rt;
    if argc != 1 {
        crate::util::die("len() expects 1 argument");
    }
    match argv[0].py_type {
        PyType::Str => {
            if let Some(ref s) = argv[0].as_str {
                return PyValue::new_int(s.len() as i64);
            }
        }
        PyType::List => {
            if let Some(ref l) = argv[0].as_list {
                return PyValue::new_int(l.items.len() as i64);
            }
        }
        _ => {}
    }
    crate::util::die("len() unsupported for this type");
    PyValue::new_none()
}

fn builtin_int(rt: &mut PyRuntime, argc: usize, argv: &mut [PyValue]) -> PyValue {
    let _ = rt;
    if argc != 1 {
        crate::util::die("int() expects 1 argument");
    }
    match argv[0].py_type {
        PyType::Int => argv[0].clone(),
        PyType::Float => PyValue::new_int(argv[0].as_float as i64),
        PyType::Bool => PyValue::new_int(if argv[0].as_bool { 1 } else { 0 }),
        _ => {
            crate::util::die("int() unsupported for this type");
            PyValue::new_none()
        }
    }
}

fn builtin_float(rt: &mut PyRuntime, argc: usize, argv: &mut [PyValue]) -> PyValue {
    let _ = rt;
    if argc != 1 {
        crate::util::die("float() expects 1 argument");
    }
    PyValue::new_float(py_as_number(&argv[0]))
}

fn builtin_str(rt: &mut PyRuntime, argc: usize, argv: &mut [PyValue]) -> PyValue {
    let _ = rt;
    if argc != 1 {
        crate::util::die("str() expects 1 argument");
    }
    let s = py_to_string(&argv[0]);
    PyValue::new_string(s)
}

fn builtin_bool(rt: &mut PyRuntime, argc: usize, argv: &mut [PyValue]) -> PyValue {
    let _ = rt;
    if argc != 1 {
        crate::util::die("bool() expects 1 argument");
    }
    PyValue::new_bool(py_is_truthy(&argv[0]))
}

fn builtin_import(rt: &mut PyRuntime, argc: usize, argv: &mut [PyValue]) -> PyValue {
    let _ = rt;
    if argc != 1 {
        crate::util::die("__import__() expects one string argument");
    }
    if let Some(ref s) = argv[0].as_str {
        if s == "sys" {
            return rt.sys_value.clone();
        }
        crate::util::die(&format!("unsupported import: {}", s));
    }
    crate::util::die("__import__() expects one string argument");
    PyValue::new_none()
}

fn builtin_run_path(rt: &mut PyRuntime, argc: usize, argv: &mut [PyValue]) -> PyValue {
    if argc != 1 {
        crate::util::die("run_path() expects one string argument");
    }
    if let Some(ref path) = argv[0].as_str {
        return builtin_run_path_impl(rt, path);
    }
    crate::util::die("run_path() expects one string argument");
    PyValue::new_none()
}

pub fn builtin_run_path_impl(rt: &mut PyRuntime, path: &str) -> PyValue {
    use std::fs;

    let source = fs::read_to_string(path).expect(&format!("cannot open {}", path));
    let module = crate::ast::parse_source(&source, path);

    let old_argv = rt.sys_value.as_sys.as_ref().unwrap().argv.clone();
    let mut new_argv = PyValue::new_list();
    if let Some(ref mut new_l) = new_argv.as_list {
        if let Some(ref old_l) = old_argv.as_list {
            for i in 1..old_l.items.len() {
                new_l.items.push(old_l.items[i].clone());
            }
        }
    }
    rt.sys_value = PyValue::new_sys(new_argv.clone());

    let mut module_env = PyEnv::new(Some(PyEnv {
        parent: Some(Box::new(rt.globals.clone())),
        items: Vec::new(),
    }));
    module_env.set("__name__", PyValue::new_string("__main__".to_string()));
    module_env.set("__file__", PyValue::new_string(path.to_string()));

    let result = crate::eval::exec_module(rt, &mut module_env, &module);

    rt.sys_value = old_argv;

    if result.has_return {
        if let Some(v) = result.value {
            return v;
        }
    }
    PyValue::new_none()
}

pub fn py_is_truthy(value: &PyValue) -> bool {
    match value.py_type {
        PyType::None => false,
        PyType::Bool => value.as_bool,
        PyType::Int => value.as_int != 0,
        PyType::Float => value.as_float != 0.0,
        PyType::Str => {
            if let Some(ref s) = value.as_str {
                !s.is_empty()
            } else {
                false
            }
        }
        PyType::List => {
            if let Some(ref l) = value.as_list {
                !l.items.is_empty()
            } else {
                false
            }
        }
        PyType::Sys | PyType::Function | PyType::BuiltinFunction => true,
    }
}

pub fn py_as_number(value: &PyValue) -> f64 {
    match value.py_type {
        PyType::Int => value.as_int as f64,
        PyType::Float => value.as_float,
        PyType::Bool => if value.as_bool { 1.0 } else { 0.0 },
        _ => {
            crate::util::die("expected number");
            0.0
        }
    }
}

pub fn py_to_string(value: &PyValue) -> String {
    match value.py_type {
        PyType::None => "None".to_string(),
        PyType::Bool => if value.as_bool { "True".to_string() } else { "False".to_string() },
        PyType::Int => value.as_int.to_string(),
        PyType::Float => value.as_float.to_string(),
        PyType::Str => {
            if let Some(ref s) = value.as_str {
                s.clone()
            } else {
                String::new()
            }
        }
        PyType::List => "<list>".to_string(),
        PyType::Sys => "<sys>".to_string(),
        PyType::Function => "<function>".to_string(),
        PyType::BuiltinFunction => "<builtin-function>".to_string(),
    }
}

impl fmt::Display for PyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", py_to_string(self))
    }
}