"""
cpy0i.py - A small Python interpreter written in Python.
Usage: python cpy0i.py <script.py> [args...]

This is a deliberately simplified interpreter. It supports a compact subset:
- expressions, assignments
- if / while
- function definitions, calls, return
- basic arithmetic and comparisons

It uses Python's standard ast module directly and avoids extra module
dependencies such as custom parsers or regex-based tokenizers.
"""

import ast
import operator
import sys


class Environment:
    def __init__(self, parent=None):
        self.vars = {}
        self.parent = parent

    def get(self, name):
        if name in self.vars:
            return self.vars[name]
        if self.parent is not None:
            return self.parent.get(name)
        raise NameError(f"name '{name}' is not defined")

    def set(self, name, value):
        self.vars[name] = value

    def assign(self, name, value):
        env = self._find(name)
        if env is None:
            self.vars[name] = value
        else:
            env.vars[name] = value

    def _find(self, name):
        if name in self.vars:
            return self
        if self.parent is not None:
            return self.parent._find(name)
        return None


class ReturnBox:
    __slots__ = ("value",)

    def __init__(self, value):
        self.value = value


class PyFunction:
    def __init__(self, node, closure, interpreter):
        self.node = node
        self.closure = closure
        self.interpreter = interpreter
        self.name = node.name

    def __call__(self, *args):
        params = self.node.args.args
        if len(args) != len(params):
            raise TypeError(
                f"{self.name}() takes {len(params)} positional arguments but {len(args)} were given"
            )

        local_env = Environment(self.closure)
        for param, arg in zip(params, args):
            local_env.set(param.arg, arg)

        for stmt in self.node.body:
            result = self.interpreter.exec_stmt(stmt, local_env)
            if isinstance(result, ReturnBox):
                return result.value
        return None

    def __repr__(self):
        return f"<function {self.name}>"


class Interpreter:
    def __init__(self, script_argv=None):
        self.global_env = Environment()
        self._setup_builtins(script_argv or [])

    def _setup_builtins(self, script_argv):
        builtins = {
            "print": print,
            "len": len,
            "range": range,
            "int": int,
            "float": float,
            "str": str,
            "bool": bool,
            "abs": abs,
            "min": min,
            "max": max,
            "sum": sum,
            "None": None,
            "True": True,
            "False": False,
        }
        for name, value in builtins.items():
            self.global_env.set(name, value)

        self.global_env.set("__name__", "__main__")
        self.global_env.set("__file__", script_argv[0] if script_argv else None)
        self.global_env.set("__builtins__", {})
        self.global_env.set("sys", SimpleSys(script_argv))

    def exec_file(self, path):
        with open(path, "r", encoding="utf-8") as f:
            source = f.read()
        tree = ast.parse(source, filename=path)
        self.exec_module(tree, self.global_env)

    def exec_module(self, tree, env):
        for stmt in tree.body:
            result = self.exec_stmt(stmt, env)
            if isinstance(result, ReturnBox):
                return result
        return None

    def exec_stmt(self, node, env):
        t = type(node)

        if t is ast.Expr:
            self.eval_expr(node.value, env)
            return None

        if t is ast.Assign:
            value = self.eval_expr(node.value, env)
            for target in node.targets:
                self.assign_target(target, value, env)
            return None

        if t is ast.If:
            branch = node.body if self.eval_expr(node.test, env) else node.orelse
            for stmt in branch:
                result = self.exec_stmt(stmt, env)
                if isinstance(result, ReturnBox):
                    return result
            return None

        if t is ast.While:
            while self.eval_expr(node.test, env):
                for stmt in node.body:
                    result = self.exec_stmt(stmt, env)
                    if isinstance(result, ReturnBox):
                        return result
            return None

        if t is ast.FunctionDef:
            env.set(node.name, PyFunction(node, env, self))
            return None

        if t is ast.Return:
            value = self.eval_expr(node.value, env) if node.value is not None else None
            return ReturnBox(value)

        if t is ast.Pass:
            return None

        raise NotImplementedError(f"statement not supported: {t.__name__}")

    def assign_target(self, target, value, env):
        if isinstance(target, ast.Name):
            env.assign(target.id, value)
            return
        raise NotImplementedError(f"assignment target not supported: {type(target).__name__}")

    def eval_expr(self, node, env):
        t = type(node)

        if t is ast.Constant:
            return node.value

        if t is ast.Name:
            return env.get(node.id)

        if t is ast.BinOp:
            left = self.eval_expr(node.left, env)
            right = self.eval_expr(node.right, env)
            return self.apply_binop(node.op, left, right)

        if t is ast.UnaryOp:
            value = self.eval_expr(node.operand, env)
            return self.apply_unaryop(node.op, value)

        if t is ast.Compare:
            left = self.eval_expr(node.left, env)
            for op, comparator in zip(node.ops, node.comparators):
                right = self.eval_expr(comparator, env)
                if not self.apply_cmpop(op, left, right):
                    return False
                left = right
            return True

        if t is ast.Call:
            func = self.eval_expr(node.func, env)
            args = [self.eval_expr(arg, env) for arg in node.args]
            if node.keywords:
                raise NotImplementedError("keyword arguments are not supported")
            return func(*args)

        raise NotImplementedError(f"expression not supported: {t.__name__}")

    def apply_binop(self, op, left, right):
        ops = {
            ast.Add: operator.add,
            ast.Sub: operator.sub,
            ast.Mult: operator.mul,
            ast.Div: operator.truediv,
            ast.FloorDiv: operator.floordiv,
            ast.Mod: operator.mod,
            ast.Pow: operator.pow,
        }
        fn = ops.get(type(op))
        if fn is None:
            raise NotImplementedError(f"binary op not supported: {type(op).__name__}")
        return fn(left, right)

    def apply_unaryop(self, op, value):
        if isinstance(op, ast.UAdd):
            return +value
        if isinstance(op, ast.USub):
            return -value
        if isinstance(op, ast.Not):
            return not value
        raise NotImplementedError(f"unary op not supported: {type(op).__name__}")

    def apply_cmpop(self, op, left, right):
        ops = {
            ast.Eq: operator.eq,
            ast.NotEq: operator.ne,
            ast.Lt: operator.lt,
            ast.LtE: operator.le,
            ast.Gt: operator.gt,
            ast.GtE: operator.ge,
        }
        fn = ops.get(type(op))
        if fn is None:
            raise NotImplementedError(f"compare op not supported: {type(op).__name__}")
        return fn(left, right)


class SimpleSys:
    def __init__(self, argv):
        self.argv = list(argv)


def main():
    if len(sys.argv) < 2:
        print("Usage: python cpy0i.py <script.py> [args...]", file=sys.stderr)
        raise SystemExit(1)

    script_path = sys.argv[1]
    script_argv = sys.argv[1:]
    interp = Interpreter(script_argv=script_argv)
    interp.exec_file(script_path)


if __name__ == "__main__":
    main()
