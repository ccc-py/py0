mod ast;
mod ast_nodes;
mod call;
mod env;
mod eval;
mod exception;
mod function;
mod lexer;
mod operator;
mod parser;
mod util;
mod value;

use std::env as std_env;
use std::fs;

use ast::parse_source;
use eval::exec_module;
use value::{PyRuntime, PyValue, PyEnv};

fn main() {
    let args: Vec<String> = std_env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: py0i <script.py>");
        std::process::exit(1);
    }

    let mut rt = PyRuntime::new();
    let argv_refs: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
    rt.set_argv(args.len() - 1, &argv_refs);

    let source = fs::read_to_string(&args[1]).expect(&format!("cannot open {}", args[1]));
    let module = parse_source(&source, &args[1]);

    rt.globals.set("__name__", PyValue::new_string("__main__".to_string()));
    rt.globals.set("__file__", PyValue::new_string(args[1].clone()));

    let globals_ptr = &mut rt.globals as *mut PyEnv;
    let _result = unsafe {
        exec_module(&mut rt, &mut *globals_ptr, &module)
    };
}