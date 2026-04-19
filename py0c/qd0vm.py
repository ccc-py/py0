#!/usr/bin/env python3
"""qd0vm.py — QD0 IR Virtual Machine"""
import sys
import re
import importlib
import os

OP = 0
A1 = 1
A2 = 2
RR = 3
LN = 4

_TOKEN_RE = re.compile(r"'[^']*'|\"[^\"]*\"|\S+")
_LABEL_RE  = re.compile(r'^[A-Za-z_]\w*:{1,2}$')

def parse_value(s):
    if s == '_':
        return None
    if s == 'True' or s == 'true':
        return True
    if s == 'False' or s == 'false':
        return False
    if s == 'None' or s == 'null':
        return None
    if not s:
        return s
    c0 = s[0]
    if c0 == "'" or c0 == '"':
        val = s[1:-1]
        result = []
        i = 0
        n = len(val)
        while i < n:
            if val[i] == '\\' and i + 1 < n:
                nxt = val[i + 1]
                if nxt == 'n':
                    result.append('\n')
                    i = i + 2
                elif nxt == 't':
                    result.append('\t')
                    i = i + 2
                elif nxt == 'r':
                    result.append('\r')
                    i = i + 2
                elif nxt == '\\':
                    result.append('\\')
                    i = i + 2
                elif nxt == '"':
                    result.append('"')
                    i = i + 2
                elif nxt == "'":
                    result.append("'")
                    i = i + 2
                else:
                    result.append(val[i])
                    i = i + 1
            else:
                result.append(val[i])
                i = i + 1
        return ''.join(result)
    try:
        return int(s)
    except ValueError:
        pass
    try:
        return float(s)
    except ValueError:
        pass
    return s


def tokenize_line(line):
    tokens = []
    i = 0
    n = len(line)
    while n > 0 and (line[n - 1] == '\n' or line[n - 1] == '\r'):
        n = n - 1
    while i < n:
        c = line[i]
        if c == ' ' or c == '\t':
            i = i + 1
            continue
        if c == ';':
            break
        if c == "'" or c == '"':
            q = c
            buf = [c]
            i = i + 1
            while i < n:
                ch = line[i]
                if ch == '\\' and i + 1 < n:
                    buf.append(ch)
                    buf.append(line[i + 1])
                    i = i + 2
                    continue
                if ch == q:
                    buf.append(ch)
                    i = i + 1
                    break
                buf.append(ch)
                i = i + 1
            tokens.append(''.join(buf))
        else:
            buf = []
            while i < n:
                ch = line[i]
                if ch == ' ' or ch == '\t' or ch == ';':
                    break
                buf.append(ch)
                i = i + 1
            if buf:
                tokens.append(''.join(buf))
    return tokens


def parse_line(line):
    line = line.strip()
    if not line or line[0] == ';':
        return None
    if _LABEL_RE.match(line):
        return ('LABEL', line.rstrip(':'), '_', '_')
    tokens = tokenize_line(line)
    if not tokens:
        return None
    while len(tokens) < 4:
        tokens.append('_')
    return (tokens[0].upper(), tokens[1], tokens[2], tokens[3])


def load_program(path):
    instructions = []
    label_map = {}
    function_map = {}
    class_map = {}
    func_stack = []
    cls_stack = []
    f = open(path, 'r')
    lineno = 0
    idx = 0
    for raw in f:
        lineno = lineno + 1
        parsed = parse_line(raw)
        if parsed is None:
            continue
        op = parsed[0]
        a1 = parsed[1]
        a2 = parsed[2]
        r  = parsed[3]
        inst = [op, a1, a2, r, lineno]
        instructions.append(inst)
        if op == 'LABEL':
            label_map[a1] = idx
        elif op == 'FUNCTION':
            func_stack.append((r, idx))
        elif op == 'FUNCTION_END':
            if func_stack:
                p = func_stack.pop()
                function_map[p[0]] = (p[1], idx)
        elif op == 'CLASS':
            cls_stack.append((a1, idx))
        elif op == 'CLASS_END':
            if cls_stack:
                p = cls_stack.pop()
                class_map[p[0]] = (p[1], idx)
        idx = idx + 1
    f.close()
    return instructions, label_map, function_map, class_map


class Frame:
    def __init__(self, name, instructions, label_map, globs):
        self.name = name
        self.instructions = instructions
        self.label_map = label_map
        self.globs = globs
        self.locs = {}
        self.pc = 0
        self.arg_buf = []
        self.list_buf = []
        self.tuple_buf = []
        self.set_buf = []
        self.dict_buf = []
        self.exhaust = {}
        self.try_stack = []
        self.current_exc = None

    def resolve(self, token):
        if not isinstance(token, str):
            return token
        if token == '_':
            return None
        if token in self.locs:
            return self.locs[token]
        if token in self.globs:
            return self.globs[token]
        val = parse_value(token)
        if type(val) is str and val == token:
            raise NameError('未定義: ' + token)
        return val

    def setv(self, name, value):
        if name and name != '_':
            self.locs[name] = value


class QD0VM:
    def __init__(self, instructions, label_map, function_map, class_map, script_argv=None):
        self.instructions = instructions
        self.label_map = label_map
        self.function_map = function_map
        self.class_map = class_map

        import types
        ss = types.SimpleNamespace()
        for attr in ('stdin', 'stdout', 'stderr', 'exit', 'path', 'platform', 'version'):
            try:
                setattr(ss, attr, getattr(sys, attr))
            except Exception:
                pass
        if script_argv is not None:
            ss.argv = list(script_argv)
        else:
            ss.argv = list(sys.argv)

        self.globs = {
            'print': print, 'len': len, 'range': range,
            'int': int, 'float': float, 'str': str, 'bool': bool,
            'list': list, 'tuple': tuple, 'dict': dict, 'set': set,
            'abs': abs, 'max': max, 'min': min, 'sum': sum,
            'repr': repr, 'type': type, 'isinstance': isinstance,
            'issubclass': issubclass, 'hasattr': hasattr,
            'getattr': getattr, 'setattr': setattr, 'delattr': delattr,
            'callable': callable, 'iter': iter, 'next': next,
            'enumerate': enumerate, 'zip': zip, 'map': map, 'filter': filter,
            'sorted': sorted, 'reversed': reversed,
            'open': open, 'input': input,
            'id': id, 'hash': hash, 'hex': hex, 'oct': oct, 'bin': bin,
            'chr': chr, 'ord': ord, 'round': round, 'pow': pow,
            'divmod': divmod, 'format': format, 'vars': vars, 'dir': dir,
            'any': any, 'all': all, 'object': object, 'super': super,
            'property': property, 'staticmethod': staticmethod,
            'classmethod': classmethod,
            'Exception': Exception, 'ValueError': ValueError,
            'TypeError': TypeError, 'KeyError': KeyError,
            'IndexError': IndexError, 'AttributeError': AttributeError,
            'NameError': NameError, 'RuntimeError': RuntimeError,
            'StopIteration': StopIteration,
            'NotImplementedError': NotImplementedError,
            'OSError': OSError, 'IOError': IOError,
            'FileNotFoundError': FileNotFoundError,
            'ImportError': ImportError, 'AssertionError': AssertionError,
            'ZeroDivisionError': ZeroDivisionError,
            'OverflowError': OverflowError,
            'ArithmeticError': ArithmeticError,
            'LookupError': LookupError,
            'PermissionError': PermissionError,
            'TimeoutError': TimeoutError,
            'True': True, 'False': False, 'None': None,
            '__name__': '__main__', '__file__': '<qd0>',
            '__import__': __import__,
            'sys': ss, 're': re, 'os': os,
            'types': __import__('types'),
            'importlib': importlib,
            'traceback': __import__('traceback'),
        }

        for fname in function_map:
            se = function_map[fname]
            self.globs[fname] = self._make_fn(fname, se[0], se[1])

        for cname in class_map:
            se = class_map[cname]
            self.globs[cname] = self._make_class(cname, se[0], se[1])

        self._skip = set()
        for fname in function_map:
            se = function_map[fname]
            for i in range(se[0], se[1] + 1):
                self._skip.add(i)
        for cname in class_map:
            se = class_map[cname]
            for i in range(se[0], se[1] + 1):
                self._skip.add(i)

    def _make_class(self, cname, start_idx, end_idx):
        instructions = self.instructions
        # 往 CLASS 指令之前找父類
        bases = []
        for i in range(max(0, start_idx - 8), start_idx):
            inst = instructions[i]
            if inst[OP] == 'LOAD_NAME' and inst[A1] in self.globs:
                candidate = self.globs[inst[A1]]
                if isinstance(candidate, type) and candidate is not type:
                    bases.append(candidate)
        base_tuple = tuple(bases) if bases else (object,)
        methods = {}
        for base in reversed(base_tuple):
            for k, v in vars(base).items():
                if not k.startswith('__'):
                    methods[k] = v
        cf = Frame(cname, instructions, self.label_map, self.globs)
        idx = start_idx + 1
        while idx <= end_idx:
            inst = instructions[idx]
            op = inst[OP]
            if op == 'FUNCTION':
                fname = inst[RR]
                depth = 1
                j = idx + 1
                while j <= end_idx:
                    if instructions[j][OP] == 'FUNCTION':
                        depth = depth + 1
                    elif instructions[j][OP] == 'FUNCTION_END':
                        depth = depth - 1
                        if depth == 0:
                            break
                    j = j + 1
                methods[fname] = self._make_fn(fname, idx, j)
                idx = j + 1
                continue
            if op not in ('CLASS', 'CLASS_END', 'ENTER_SCOPE', 'EXIT_SCOPE',
                          'PARAM', 'VARARG', 'KWARG', 'LABEL', 'PASS', '#'):
                try:
                    self._exec(inst, cf)
                    if op == 'STORE' and inst[RR] != '_':
                        methods[inst[RR]] = cf.locs.get(inst[RR])
                except Exception:
                    pass
            idx = idx + 1
        return type(cname, base_tuple, methods)

    def _make_fn(self, fname, start_idx, end_idx, captured_env=None):
        vm = self
        def qd_fn(*args):
            return vm._call_fn(fname, start_idx, end_idx, args, captured_env)
        qd_fn.__name__ = fname
        return qd_fn

    def _call_fn(self, fname, start_idx, end_idx, args, captured_env=None):
        frame = Frame(fname, self.instructions, self.label_map, self.globs)
        if captured_env:
            for k, v in captured_env.items():
                frame.locs[k] = v
        pc = start_idx + 1
        pi = 0
        instructions = self.instructions
        while pc <= end_idx:
            inst = instructions[pc]
            op = inst[OP]
            if op == 'ENTER_SCOPE' or op == 'PASS':
                pc = pc + 1
                continue
            if op == 'PARAM':
                if pi < len(args):
                    frame.locs[inst[RR]] = args[pi]
                else:
                    frame.locs[inst[RR]] = None
                pi = pi + 1
                pc = pc + 1
                continue
            if op == 'VARARG':
                frame.locs[inst[RR]] = tuple(args[pi:])
                pc = pc + 1
                continue
            if op == 'KWARG':
                frame.locs[inst[RR]] = {}
                pc = pc + 1
                continue
            break
        frame.pc = pc
        # 計算 inner_skip（跳過 nested function/class body，保留 FUNCTION 指令本身）
        inner_skip = set()
        i = start_idx + 1
        while i <= end_idx:
            inst = instructions[i]
            if inst[OP] == 'FUNCTION':
                depth = 1
                j = i + 1
                while j <= end_idx:
                    if instructions[j][OP] == 'FUNCTION':
                        depth = depth + 1
                    elif instructions[j][OP] == 'FUNCTION_END':
                        depth = depth - 1
                        if depth == 0:
                            break
                    j = j + 1
                for k in range(i + 1, j + 1):
                    inner_skip.add(k)
                i = j + 1
                continue
            elif inst[OP] == 'CLASS':
                depth = 1
                j = i + 1
                while j <= end_idx:
                    if instructions[j][OP] == 'CLASS':
                        depth = depth + 1
                    elif instructions[j][OP] == 'CLASS_END':
                        depth = depth - 1
                        if depth == 0:
                            break
                    j = j + 1
                for k in range(i + 1, j + 1):
                    inner_skip.add(k)
                i = j + 1
                continue
            i = i + 1
        if inner_skip:
            return self._run(frame, end_idx, inner_skip)
        return self._run(frame, end_idx)

    def run(self):
        frame = Frame('__main__', self.instructions, self.label_map, self.globs)
        frame.pc = 0
        self._run(frame, len(self.instructions) - 1, self._skip)

    def _run(self, frame, end_idx, skip=None):
        instructions = self.instructions
        label_map = self.label_map
        SKIP_OPS = ('FUNCTION_END', 'ENTER_SCOPE', 'EXIT_SCOPE',
                    'PARAM', 'VARARG', 'KWARG', 'LABEL',
                    'CLASS', 'CLASS_END', 'PASS', '#')
        _exec = self._exec
        while frame.pc <= end_idx:
            idx = frame.pc
            if skip and idx in skip:
                frame.pc = idx + 1
                continue
            inst = instructions[idx]
            if inst[OP] in SKIP_OPS:
                frame.pc = idx + 1
                continue
            try:
                ret = _exec(inst, frame)
            except SystemExit:
                raise
            except Exception as e:
                if frame.try_stack:
                    h = frame.try_stack.pop()
                    frame.current_exc = e
                    if h in label_map:
                        frame.pc = label_map[h]
                        continue
                raise
            if ret is None:
                frame.pc = idx + 1
                continue
            if ret[0] == 'R':
                return ret[1]
            if ret[0] == 'J':
                lbl = ret[1]
                if lbl not in label_map:
                    raise RuntimeError('未知標籤: ' + str(lbl))
                frame.pc = label_map[lbl]
                continue
            frame.pc = idx + 1
        return None

    def _exec(self, inst, frame):
        op  = inst[OP]
        a1  = inst[A1]
        a2  = inst[A2]
        r   = inst[RR]
        locs  = frame.locs
        globs = frame.globs
        resolve = frame.resolve
        setv = frame.setv

        def V(t):
            if t is None or t == '_':
                return None
            return resolve(t)

        def lit(t):
            if t is None or t == '_':
                return None
            return parse_value(t)

        # ── Nested FUNCTION（closure）────────────────────────────────────────
        if op == 'FUNCTION':
            fname2 = r
            if fname2 in self.function_map:
                se = self.function_map[fname2]
                fn = self._make_fn(fname2, se[0], se[1], dict(locs))
                locs[fname2] = fn
                globs[fname2] = fn
            return None

        # ── Load / Store ─────────────────────────────────────────────────────
        elif op == 'LOAD_CONST':
            setv(r, lit(a1))
        elif op == 'LOAD_NAME':
            setv(r, resolve(a1))
        elif op == 'LOAD_ATTR':
            setv(r, getattr(V(a1), a2))
        elif op == 'STORE':
            val = V(a1)
            setv(r, val)
            if frame.name == '__main__':
                globs[r] = val
        elif op == 'STORE_ATTR':
            setattr(V(a1), a2, V(r))
        elif op == 'DELETE_NAME':
            locs.pop(a1, None)
            globs.pop(a1, None)

        # ── Arithmetic ───────────────────────────────────────────────────────
        elif op == 'ADD':
            setv(r, V(a1) + V(a2))
        elif op == 'SUB':
            setv(r, V(a1) - V(a2))
        elif op == 'MUL':
            setv(r, V(a1) * V(a2))
        elif op == 'DIV':
            setv(r, V(a1) / V(a2))
        elif op == 'FLOOR_DIV':
            setv(r, V(a1) // V(a2))
        elif op == 'MOD':
            setv(r, V(a1) % V(a2))
        elif op == 'POW':
            setv(r, V(a1) ** V(a2))
        elif op == 'NEG':
            setv(r, -V(a1))
        elif op == 'POS':
            setv(r, +V(a1))
        elif op == 'BIT_AND':
            setv(r, V(a1) & V(a2))
        elif op == 'BIT_OR':
            setv(r, V(a1) | V(a2))
        elif op == 'BIT_XOR':
            setv(r, V(a1) ^ V(a2))
        elif op == 'BIT_NOT':
            setv(r, ~V(a1))
        elif op == 'LSHIFT':
            setv(r, V(a1) << V(a2))
        elif op == 'RSHIFT':
            setv(r, V(a1) >> V(a2))
        elif op == 'BINOP':
            setv(r, V(a1) + V(a2))

        # ── Compare ──────────────────────────────────────────────────────────
        elif op == 'CMP_EQ':
            setv(r, V(a1) == V(a2))
        elif op == 'CMP_NE':
            setv(r, V(a1) != V(a2))
        elif op == 'CMP_LT':
            setv(r, V(a1) < V(a2))
        elif op == 'CMP_LE':
            setv(r, V(a1) <= V(a2))
        elif op == 'CMP_GT':
            setv(r, V(a1) > V(a2))
        elif op == 'CMP_GE':
            setv(r, V(a1) >= V(a2))
        elif op == 'CMP_IS':
            setv(r, V(a1) is V(a2))
        elif op == 'CMP_IS_NOT':
            setv(r, V(a1) is not V(a2))
        elif op == 'CMP_IN':
            setv(r, V(a1) in V(a2))
        elif op == 'CMP_NOT_IN':
            setv(r, V(a1) not in V(a2))
        elif op == 'CMP':
            setv(r, V(a1) > V(a2))

        # ── Boolean ──────────────────────────────────────────────────────────
        elif op == 'AND':
            setv(r, V(a1) and V(a2))
        elif op == 'OR':
            setv(r, V(a1) or V(a2))
        elif op == 'NOT':
            setv(r, not V(a1))

        # ── Control Flow ─────────────────────────────────────────────────────
        elif op == 'JUMP':
            return ('J', a1)
        elif op == 'BRANCH_IF_TRUE':
            if V(a1):
                return ('J', r)
        elif op == 'BRANCH_IF_FALSE':
            if not V(a1):
                return ('J', r)
        elif op == 'BREAK':
            return ('J', r)
        elif op == 'CONTINUE':
            return ('J', r)

        # ── Iteration ────────────────────────────────────────────────────────
        elif op == 'GET_ITER':
            setv(r, iter(V(a1)))
        elif op == 'ITER_NEXT':
            it = resolve(a1)
            try:
                setv(r, next(it))
                frame.exhaust[a1] = False
            except StopIteration:
                frame.exhaust[a1] = True
                setv(r, None)
        elif op == 'BRANCH_IF_EXHAUST':
            if frame.exhaust.get(a1, False):
                return ('J', r)
        elif op == 'UNPACK_ITER':
            setv(r, iter(V(a1)))
            frame.exhaust[r] = False

        # ── Collections ──────────────────────────────────────────────────────
        elif op == 'TUPLE_APPEND':
            frame.tuple_buf.append((lit(a2), V(a1)))
        elif op == 'BUILD_TUPLE':
            count = int(lit(a1))
            raw = frame.tuple_buf[-count:]
            del frame.tuple_buf[-count:]
            raw.sort(key=lambda x: 999999 if x[0] is None else x[0])
            setv(r, tuple(x[1] for x in raw))
        elif op == 'LIST_APPEND':
            frame.list_buf.append((lit(a2), V(a1)))
        elif op == 'BUILD_LIST':
            count = int(lit(a1))
            raw = frame.list_buf[-count:]
            del frame.list_buf[-count:]
            raw.sort(key=lambda x: 999999 if x[0] is None else x[0])
            lst = []
            for x in raw:
                lst.append(x[1])
            setv(r, lst)
        elif op == 'SET_APPEND':
            frame.set_buf.append(V(a1))
        elif op == 'BUILD_SET':
            count = int(lit(a1))
            items = frame.set_buf[-count:]
            del frame.set_buf[-count:]
            setv(r, set(items))
        elif op == 'DICT_INSERT':
            frame.dict_buf.append((V(a1), V(a2)))
        elif op == 'BUILD_DICT':
            count = int(lit(a1))
            items = frame.dict_buf[-count:]
            del frame.dict_buf[-count:]
            d = {}
            for kv in items:
                d[kv[0]] = kv[1]
            setv(r, d)
        elif op == 'DICT_UPDATE':
            pass

        # ── Subscript ────────────────────────────────────────────────────────
        elif op == 'SUBSCRIPT':
            setv(r, V(a1)[self._key(a2, frame)])
        elif op == 'SUBSCRIPT_SET':
            V(a1)[self._key(a2, frame)] = V(r)

        # ── Function / Call ──────────────────────────────────────────────────
        elif op == 'ARG_PUSH':
            frame.arg_buf.append((lit(a2), V(a1)))
        elif op == 'CALL':
            func = V(a1)
            if a2 and a2 != '_':
                argc = int(lit(a2))
            else:
                argc = len(frame.arg_buf)
            raw = frame.arg_buf[-argc:]
            del frame.arg_buf[-argc:]
            raw.sort(key=lambda x: 999999 if x[0] is None else x[0])
            args = []
            for x in raw:
                args.append(x[1])
            setv(r, func(*args))
        elif op == 'RETURN':
            return ('R', V(a1))
        elif op == 'MAKE_CLOSURE':
            setv(r, resolve(a1))

        # ── Import ───────────────────────────────────────────────────────────
        elif op == 'IMPORT':
            if a1 not in globs:
                try:
                    mod = importlib.import_module(a1)
                except ImportError:
                    mod = None
                globs[a1] = mod
            setv(a1, globs[a1])
        elif op == 'IMPORT_FROM':
            try:
                mod = importlib.import_module(a1)
                val = getattr(mod, a2)
            except Exception:
                val = None
            globs[a2] = val
            setv(a2, val)
        elif op == 'IMPORT_STAR':
            try:
                mod = importlib.import_module(a1)
                for k in dir(mod):
                    if not k.startswith('_'):
                        globs[k] = getattr(mod, k)
            except Exception:
                pass

        # ── Exception ────────────────────────────────────────────────────────
        elif op == 'TRY_BEGIN':
            frame.try_stack.append(a1)
        elif op == 'TRY_END':
            if frame.try_stack:
                frame.try_stack.pop()
        elif op == 'RAISE':
            exc = V(a1)
            if exc is None:
                exc = frame.current_exc or Exception()
            raise exc
        elif op == 'RAISE_REUSE':
            raise frame.current_exc or Exception()
        elif op == 'MATCH_EXC':
            et = V(a1)
            if et:
                setv(r, isinstance(frame.current_exc, et))
            else:
                setv(r, True)
        elif op == 'EXCEPT_VAR':
            setv(r, frame.current_exc)

        # ── With ─────────────────────────────────────────────────────────────
        elif op == 'WITH_ENTER':
            mgr = V(a1)
            val = mgr.__enter__()
            if r and r != '_':
                setv(r, val)
                globs[r] = val
        elif op == 'WITH_EXIT':
            pass

        # ── Assert ───────────────────────────────────────────────────────────
        elif op == 'ASSERT':
            if not V(a1):
                raise AssertionError()
        elif op == 'ASSERT_MSG':
            pass

        # ── Ternary ──────────────────────────────────────────────────────────
        elif op == 'TERNARY':
            if V(a1):
                setv(r, V(a2))
            else:
                setv(r, None)

        # ── FString ─────────────────────────────────────────────────────────
        elif op == 'FSTRING_START':
            frame.fstring_buf = []
        elif op == 'FSTRING_PART':
            part = a1
            if part == '_' or not part:
                pass
            elif (part.startswith("'") and part.endswith("'")) or (part.startswith('"') and part.endswith('"')):
                frame.fstring_buf.append(part[1:-1])
            else:
                frame.fstring_buf.append(str(V(part)))
        elif op == 'FSTRING':
            result = ''.join(frame.fstring_buf)
            setv(r, result)
            del frame.fstring_buf

        # ── Type specialization ──────────────────────────────────────────────
        elif op in ('ASSUME_TYPE', 'BOX', 'UNBOX'):
            setv(r, V(a1))

        # ── Others: silently ignore ───────────────────────────────────────────
        return None

    def _key(self, key_str, frame):
        if key_str is None or key_str == '_':
            return None
        if ':' in key_str:
            parts = key_str.split(':')
            def p(s):
                if s == '_' or s == '':
                    return None
                return frame.resolve(s)
            start = p(parts[0]) if len(parts) > 0 else None
            stop  = p(parts[1]) if len(parts) > 1 else None
            step  = p(parts[2]) if len(parts) > 2 else None
            return slice(start, stop, step)
        return frame.resolve(key_str)


def main():
    if len(sys.argv) < 2:
        print('用法: python qd0vm.py <file.qd> [args...]', file=sys.stderr)
        sys.exit(1)
    qd_path = sys.argv[1]
    script_argv = sys.argv[1:]
    if not os.path.exists(qd_path):
        print("qd0vm: can't open file '" + qd_path + "'", file=sys.stderr)
        sys.exit(1)
    try:
        instructions, label_map, function_map, class_map = load_program(qd_path)
    except Exception as e:
        print('qd0vm: 載入失敗: ' + str(e), file=sys.stderr)
        sys.exit(1)
    vm = QD0VM(instructions, label_map, function_map, class_map,
               script_argv=script_argv)
    try:
        vm.run()
    except SystemExit as e:
        sys.exit(e.code)
    except Exception as e:
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == '__main__':
    main()