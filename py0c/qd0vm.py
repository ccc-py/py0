#!/usr/bin/env python3
"""
qd0vm.py — QD0 IR Virtual Machine
用法: python qd0vm.py <file.qd>
"""

import sys
import re

def parse_value(s):
    if s == '_': return None
    if s == 'True' or s == 'true': return True
    if s == 'False' or s == 'false': return False
    if s == 'None' or s == 'null': return None
    is_sq = s.startswith("'") and s.endswith("'")
    is_dq = s.startswith('"') and s.endswith('"')
    if is_sq or is_dq:
        val = s[1:-1]
        # 只替換常見 escape，避免 unicode_escape 破壞 regex pattern
        val = val.replace('\\n', '\n').replace('\\t', '\t').replace('\\r', '\r')
        val = val.replace("\\'", "'").replace('\\"', '"').replace('\\\\', '\\')
        return val
    try: return int(s)
    except ValueError: pass
    try: return float(s)
    except ValueError: pass
    return s

def tokenize_line(line):
    in_q = False
    q_char = ''
    clean = []
    i = 0
    n = len(line)
    while i < n:
        c = line[i]
        if in_q:
            if c == '\\' and i + 1 < n:
                # escaped character: keep both and skip next
                clean.append(c)
                clean.append(line[i + 1])
                i = i + 2
                continue
            if c == q_char:
                in_q = False
            clean.append(c)
        else:
            if c == '"' or c == "'":
                in_q = True
                q_char = c
                clean.append(c)
            elif c == ';':
                break
            else:
                clean.append(c)
        i = i + 1
    clean_str = ''.join(clean).strip()
    if not clean_str:
        return []
    tokens = []
    # regex: quoted string (handling \' and \") or non-whitespace
    pat = r"""'(?:[^'\\]|\\.)*'|"(?:[^"\\]|\\.)*"|\S+"""
    for m in re.finditer(pat, clean_str):
        tokens.append(m.group())
    return tokens

def parse_line(line):
    line = line.strip()
    if not line or line.startswith(';'): return None
    if re.match(r'^[A-Za-z_]\w*:{1,2}$', line):
        return ('LABEL', line.rstrip(':'), None, None)
    tokens = tokenize_line(line)
    if not tokens: return None
    while len(tokens) < 4: tokens.append('_')
    return (tokens[0].upper(), tokens[1], tokens[2], tokens[3])

class Instruction:
    def __init__(self, op, arg1, arg2, result, lineno):
        self.op = op; self.arg1 = arg1; self.arg2 = arg2
        self.result = result; self.lineno = lineno
    def __repr__(self):
        return "<" + str(self.op) + " " + str(self.arg1) + " " + str(self.arg2) + " " + str(self.result) + ">"

def load_program(path):
    instructions = []
    f = open(path, 'r')
    lineno = 1
    for raw in f:
        parsed = parse_line(raw)
        if parsed is not None:
            instructions.append(Instruction(parsed[0], parsed[1], parsed[2], parsed[3], lineno))
        lineno = lineno + 1
    f.close()

    label_map = {}
    idx = 0
    for inst in instructions:
        if inst.op == 'LABEL': label_map[inst.arg1] = idx
        idx = idx + 1

    function_map = {}
    func_stack = []
    idx = 0
    for inst in instructions:
        if inst.op == 'FUNCTION': func_stack.append((inst.result, idx))
        elif inst.op == 'FUNCTION_END':
            if len(func_stack) > 0:
                p = func_stack.pop()
                function_map[p[0]] = (p[1], idx)
        idx = idx + 1

    class_map = {}
    cls_stack = []
    idx = 0
    for inst in instructions:
        if inst.op == 'CLASS': cls_stack.append((inst.arg1, idx))
        elif inst.op == 'CLASS_END':
            if len(cls_stack) > 0:
                p = cls_stack.pop()
                class_map[p[0]] = (p[1], idx)
        idx = idx + 1

    return instructions, label_map, function_map, class_map

class Frame:
    def __init__(self, name, instructions, label_map, globals_env):
        self.name = name
        self.instructions = instructions
        self.label_map = label_map
        self.globals_env = globals_env
        self.locals = {}
        self.pc = 0
        self.return_value = None
        self.arg_buffer = []
        self.list_buffer = []
        self.tuple_buffer = []
        self.set_buffer = []
        self.dict_buffer = []
        self.exhaust = {}
        self.try_stack = []
        self.current_exc = None

    def resolve(self, token):
        if not isinstance(token, str): return token
        if token == '_': return None
        if token in self.locals: return self.locals[token]
        if token in self.globals_env: return self.globals_env[token]
        val = parse_value(token)
        if type(val) is str and val == token:
            raise NameError("未定義的名稱: " + str(token))
        return val

    def set_local(self, name, value):
        if name and name != '_': self.locals[name] = value

class QD0VM:
    def __init__(self, instructions, label_map, function_map, class_map, script_argv=None):
        self.instructions = instructions
        self.label_map = label_map
        self.function_map = function_map
        self.class_map = class_map

        # 建立腳本專屬 sys：argv 指向 script_argv
        import types
        script_sys = types.SimpleNamespace()
        for attr in dir(sys):
            if not attr.startswith('__'):
                try: setattr(script_sys, attr, getattr(sys, attr))
                except Exception: pass
        if script_argv is not None:
            script_sys.argv = list(script_argv)
        else:
            script_sys.argv = list(sys.argv)

        import importlib, os

        self.globals = {
            # builtins
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
            'id': id, 'hash': hash,
            'hex': hex, 'oct': oct, 'bin': bin, 'chr': chr, 'ord': ord,
            'round': round, 'pow': pow, 'divmod': divmod, 'format': format,
            'vars': vars, 'dir': dir, 'any': any, 'all': all,
            'object': object, 'super': super,
            'property': property, 'staticmethod': staticmethod, 'classmethod': classmethod,
            'Exception': Exception, 'ValueError': ValueError, 'TypeError': TypeError,
            'KeyError': KeyError, 'IndexError': IndexError,
            'AttributeError': AttributeError, 'NameError': NameError,
            'RuntimeError': RuntimeError, 'StopIteration': StopIteration,
            'NotImplementedError': NotImplementedError,
            'OSError': OSError, 'IOError': IOError,
            'FileNotFoundError': FileNotFoundError,
            'ImportError': ImportError, 'AssertionError': AssertionError,
            'True': True, 'False': False, 'None': None,
            '__name__': '__main__', '__file__': '<qd0>',
            'sys': script_sys, 're': re, 'os': os,
            'types': __import__('types'),
            'importlib': __import__('importlib'),
            'traceback': __import__('traceback'),
            '__import__': importlib.import_module,
        }

        for fname in function_map:
            se = function_map[fname]
            self.globals[fname] = self._make_function(fname, se[0], se[1])

        for cname in class_map:
            se = class_map[cname]
            self.globals[cname] = self._make_class(cname, se[0], se[1])

    # ── 類別工廠：執行 class body 蒐集 methods 與 class attributes ──
    def _make_class(self, cname, start_idx, end_idx):
        instructions = self.instructions
        methods = {}

        # 建立一個臨時 frame 執行 class body 的非函式指令
        class_frame = Frame(cname, instructions, self.label_map, self.globals)
        class_frame.pc = start_idx + 1   # 跳過 CLASS 本身

        idx = start_idx + 1
        while idx <= end_idx:
            inst = instructions[idx]
            if inst.op == 'FUNCTION':
                fname = inst.result
                depth = 1; j = idx + 1
                while j <= end_idx:
                    if instructions[j].op == 'FUNCTION': depth = depth + 1
                    elif instructions[j].op == 'FUNCTION_END':
                        depth = depth - 1
                        if depth == 0: break
                    j = j + 1
                methods[fname] = self._make_function(fname, idx, j)
                idx = j + 1
                continue
            # 執行非函式指令（STORE → class attribute）
            if inst.op not in ('CLASS', 'CLASS_END', 'FUNCTION', 'FUNCTION_END',
                               'ENTER_SCOPE', 'EXIT_SCOPE', 'PARAM', 'LABEL',
                               'PASS', '#'):
                try:
                    self._exec(inst, class_frame)
                    # 把 STORE 結果收進 methods
                    if inst.op == 'STORE' and inst.result and inst.result != '_':
                        val = class_frame.locals.get(inst.result)
                        methods[inst.result] = val
                except Exception:
                    pass
            idx = idx + 1

        return type(cname, (object,), methods)

    # ── 函式工廠 ──
    def _make_function(self, fname, start_idx, end_idx):
        vm = self
        def qd_function(*args):
            return vm._call_qd_function(fname, start_idx, end_idx, list(args))
        qd_function.__name__ = fname
        return qd_function

    def _call_qd_function(self, fname, start_idx, end_idx, args):
        frame = Frame(fname, self.instructions, self.label_map, self.globals)
        pc = start_idx + 1
        param_idx = 0
        while pc <= end_idx:
            inst = self.instructions[pc]
            if inst.op == 'ENTER_SCOPE' or inst.op == 'PASS':
                pc = pc + 1; continue
            if inst.op == 'PARAM':
                if param_idx < len(args):
                    frame.locals[inst.result] = args[param_idx]
                else:
                    frame.locals[inst.result] = None
                param_idx = param_idx + 1; pc = pc + 1; continue
            if inst.op == 'VARARG':
                frame.locals[inst.result] = tuple(args[param_idx:])
                pc = pc + 1; continue
            if inst.op == 'KWARG':
                frame.locals[inst.result] = {}
                pc = pc + 1; continue
            break
        frame.pc = pc
        return self._run_frame(frame, end_idx)

    def run(self):
        skip = set()
        for fname in self.function_map:
            se = self.function_map[fname]
            for i in range(se[0], se[1] + 1): skip.add(i)
        for cname in self.class_map:
            se = self.class_map[cname]
            for i in range(se[0], se[1] + 1): skip.add(i)

        frame = Frame('__main__', self.instructions, self.label_map, self.globals)
        frame.pc = 0
        self._run_frame(frame, len(self.instructions) - 1, skip_ranges=skip)

    def _run_frame(self, frame, end_idx, skip_ranges=None):
        SKIP_OPS = ('FUNCTION','FUNCTION_END','ENTER_SCOPE','EXIT_SCOPE',
                    'PARAM','VARARG','KWARG','LABEL','CLASS','CLASS_END','PASS','#')
        instructions = self.instructions

        while frame.pc <= end_idx:
            idx = frame.pc
            if skip_ranges and idx in skip_ranges:
                frame.pc = frame.pc + 1; continue

            inst = instructions[idx]
            if inst.op in SKIP_OPS:
                frame.pc = frame.pc + 1; continue

            try:
                ret = self._exec(inst, frame)
            except SystemExit:
                raise
            except Exception as e:
                if len(frame.try_stack) > 0:
                    handler = frame.try_stack[-1]
                    frame.current_exc = e
                    if handler in self.label_map:
                        frame.pc = self.label_map[handler]; continue
                raise

            if ret is None:
                frame.pc = frame.pc + 1; continue
            if ret[0] == 'RETURN': return ret[1]
            if ret[0] == 'JUMP':
                label = ret[1]
                if label not in self.label_map:
                    raise RuntimeError("未知標籤: " + str(label))
                frame.pc = self.label_map[label]; continue
            frame.pc = frame.pc + 1

        return frame.return_value

    def _exec(self, inst, frame):
        op = inst.op
        a1 = inst.arg1
        a2 = inst.arg2
        r  = inst.result

        def V(t):
            if t is None or t == '_': return None
            return frame.resolve(t)

        def lit(t):
            if t is None or t == '_': return None
            return parse_value(t)

        # ── Load / Store ──────────────────────────────────────────────
        if op == 'LOAD_CONST':
            frame.set_local(r, lit(a1))
        elif op == 'LOAD_NAME':
            frame.set_local(r, frame.resolve(a1))
        elif op == 'LOAD_ATTR':
            frame.set_local(r, getattr(V(a1), a2))
        elif op == 'STORE':
            val = V(a1)
            frame.set_local(r, val)
            frame.globals_env[r] = val
        elif op == 'STORE_ATTR':
            setattr(V(a1), a2, V(r))
        elif op == 'DELETE_NAME':
            frame.locals.pop(a1, None); frame.globals_env.pop(a1, None)

        # ── Arithmetic ───────────────────────────────────────────────
        elif op == 'ADD':       frame.set_local(r, V(a1) + V(a2))
        elif op == 'SUB':       frame.set_local(r, V(a1) - V(a2))
        elif op == 'MUL':       frame.set_local(r, V(a1) * V(a2))
        elif op == 'DIV':       frame.set_local(r, V(a1) / V(a2))
        elif op == 'FLOOR_DIV': frame.set_local(r, V(a1) // V(a2))
        elif op == 'MOD':       frame.set_local(r, V(a1) % V(a2))
        elif op == 'POW':       frame.set_local(r, V(a1) ** V(a2))
        elif op == 'NEG':       frame.set_local(r, -V(a1))
        elif op == 'POS':       frame.set_local(r, +V(a1))
        elif op == 'BIT_AND':   frame.set_local(r, V(a1) & V(a2))
        elif op == 'BIT_OR':    frame.set_local(r, V(a1) | V(a2))
        elif op == 'BIT_XOR':   frame.set_local(r, V(a1) ^ V(a2))
        elif op == 'BIT_NOT':   frame.set_local(r, ~V(a1))
        elif op == 'LSHIFT':    frame.set_local(r, V(a1) << V(a2))
        elif op == 'RSHIFT':    frame.set_local(r, V(a1) >> V(a2))
        elif op == 'BINOP':     frame.set_local(r, V(a1) + V(a2))

        # ── Compare ──────────────────────────────────────────────────
        elif op == 'CMP_EQ':     frame.set_local(r, V(a1) == V(a2))
        elif op == 'CMP_NE':     frame.set_local(r, V(a1) != V(a2))
        elif op == 'CMP_LT':     frame.set_local(r, V(a1) <  V(a2))
        elif op == 'CMP_LE':     frame.set_local(r, V(a1) <= V(a2))
        elif op == 'CMP_GT':     frame.set_local(r, V(a1) >  V(a2))
        elif op == 'CMP_GE':     frame.set_local(r, V(a1) >= V(a2))
        elif op == 'CMP_IS':     frame.set_local(r, V(a1) is V(a2))
        elif op == 'CMP_IS_NOT': frame.set_local(r, V(a1) is not V(a2))
        elif op == 'CMP_IN':     frame.set_local(r, V(a1) in V(a2))
        elif op == 'CMP_NOT_IN': frame.set_local(r, V(a1) not in V(a2))
        elif op == 'CMP':        frame.set_local(r, V(a1) > V(a2))

        # ── Boolean ──────────────────────────────────────────────────
        elif op == 'AND': frame.set_local(r, V(a1) and V(a2))
        elif op == 'OR':  frame.set_local(r, V(a1) or  V(a2))
        elif op == 'NOT': frame.set_local(r, not V(a1))

        # ── Control Flow ─────────────────────────────────────────────
        elif op == 'JUMP':
            return ('JUMP', a1)
        elif op == 'BRANCH_IF_TRUE':
            if V(a1): return ('JUMP', r)
        elif op == 'BRANCH_IF_FALSE':
            if not V(a1): return ('JUMP', r)
        elif op == 'BREAK':
            return ('JUMP', r)
        elif op == 'CONTINUE':
            return ('JUMP', r)

        # ── Iteration ────────────────────────────────────────────────
        elif op == 'GET_ITER':
            frame.set_local(r, iter(V(a1)))
        elif op == 'ITER_NEXT':
            it = frame.resolve(a1)
            try:
                frame.set_local(r, next(it))
                frame.exhaust[a1] = False
            except StopIteration:
                frame.exhaust[a1] = True
                frame.set_local(r, None)
        elif op == 'BRANCH_IF_EXHAUST':
            if frame.exhaust.get(a1, False): return ('JUMP', r)
        elif op == 'UNPACK_ITER':
            frame.set_local(r, iter(V(a1)))
            frame.exhaust[r] = False

        # ── Collections ──────────────────────────────────────────────
        elif op == 'TUPLE_APPEND':
            frame.tuple_buffer.append((lit(a2), V(a1)))
        elif op == 'BUILD_TUPLE':
            count = int(lit(a1))
            raw = frame.tuple_buffer[-count:]
            del frame.tuple_buffer[-count:]
            # 氣泡排序（不用 lambda）
            n = len(raw)
            for i in range(n):
                for j in range(0, n - i - 1):
                    vi = raw[j][0];   vj = raw[j+1][0]
                    if vi is None: vi = 999999
                    if vj is None: vj = 999999
                    if vi > vj: raw[j], raw[j+1] = raw[j+1], raw[j]
            frame.set_local(r, tuple(x[1] for x in raw))

        elif op == 'LIST_APPEND':
            frame.list_buffer.append((lit(a2), V(a1)))
        elif op == 'BUILD_LIST':
            count = int(lit(a1))
            raw = frame.list_buffer[-count:]
            del frame.list_buffer[-count:]
            n = len(raw)
            for i in range(n):
                for j in range(0, n - i - 1):
                    vi = raw[j][0];   vj = raw[j+1][0]
                    if vi is None: vi = 999999
                    if vj is None: vj = 999999
                    if vi > vj: raw[j], raw[j+1] = raw[j+1], raw[j]
            lst = []
            for x in raw: lst.append(x[1])
            frame.set_local(r, lst)

        elif op == 'SET_APPEND':
            frame.set_buffer.append(V(a1))
        elif op == 'BUILD_SET':
            count = int(lit(a1))
            items = frame.set_buffer[-count:]
            del frame.set_buffer[-count:]
            frame.set_local(r, set(items))

        elif op == 'DICT_INSERT':
            frame.dict_buffer.append((V(a1), V(a2)))
        elif op == 'BUILD_DICT':
            count = int(lit(a1))
            items = frame.dict_buffer[-count:]
            del frame.dict_buffer[-count:]
            d = {}
            for kv in items: d[kv[0]] = kv[1]
            frame.set_local(r, d)
        elif op == 'DICT_UPDATE':
            pass

        # ── Subscript ────────────────────────────────────────────────
        elif op == 'SUBSCRIPT':
            obj = V(a1)
            key = self._parse_key(a2, frame)
            frame.set_local(r, obj[key])
        elif op == 'SUBSCRIPT_SET':
            obj = V(a1)
            key = self._parse_key(a2, frame)
            obj[key] = V(r)

        # ── Function / Call ──────────────────────────────────────────
        elif op == 'ARG_PUSH':
            frame.arg_buffer.append((lit(a2), V(a1)))
        elif op == 'CALL':
            func = V(a1)
            if a2 and a2 != '_':
                argc = int(lit(a2))
            else:
                argc = len(frame.arg_buffer)
            raw = frame.arg_buffer[-argc:]
            del frame.arg_buffer[-argc:]
            # 氣泡排序
            n = len(raw)
            for i in range(n):
                for j in range(0, n - i - 1):
                    vi = raw[j][0];   vj = raw[j+1][0]
                    if vi is None: vi = 999999
                    if vj is None: vj = 999999
                    if vi > vj: raw[j], raw[j+1] = raw[j+1], raw[j]
            args = []
            for x in raw: args.append(x[1])
            frame.set_local(r, func(*args))
        elif op == 'RETURN':
            return ('RETURN', V(a1))
        elif op == 'MAKE_CLOSURE':
            frame.set_local(r, frame.resolve(a1))

        # ── Import ───────────────────────────────────────────────────
        elif op == 'IMPORT':
            import importlib
            # 若已在全域（例如 script_sys 替代品），不覆蓋
            if a1 not in frame.globals_env:
                try: mod = importlib.import_module(a1)
                except ImportError: mod = None
                frame.globals_env[a1] = mod
            frame.set_local(a1, frame.globals_env[a1])
        elif op == 'IMPORT_FROM':
            import importlib
            try:
                mod = importlib.import_module(a1)
                val = getattr(mod, a2)
            except Exception: val = None
            frame.globals_env[a2] = val
            frame.set_local(a2, val)
        elif op == 'IMPORT_STAR':
            import importlib
            try:
                mod = importlib.import_module(a1)
                for k in dir(mod):
                    if not k.startswith('_'):
                        frame.globals_env[k] = getattr(mod, k)
            except Exception: pass

        # ── Exception ────────────────────────────────────────────────
        elif op == 'TRY_BEGIN':
            frame.try_stack.append(a1)
        elif op == 'TRY_END':
            if len(frame.try_stack) > 0: frame.try_stack.pop()
        elif op == 'RAISE':
            exc = V(a1)
            if exc is None: exc = frame.current_exc or Exception()
            raise exc
        elif op == 'RAISE_REUSE':
            raise frame.current_exc or Exception()
        elif op == 'MATCH_EXC':
            exc_type = V(a1)
            if exc_type:
                frame.set_local(r, isinstance(frame.current_exc, exc_type))
            else:
                frame.set_local(r, True)
        elif op == 'EXCEPT_VAR':
            frame.set_local(r, frame.current_exc)

        # ── With ─────────────────────────────────────────────────────
        elif op == 'WITH_ENTER':
            mgr = V(a1)
            val = mgr.__enter__()
            if r and r != '_':
                frame.set_local(r, val)
                frame.globals_env[r] = val
        elif op == 'WITH_EXIT':
            pass

        # ── Assert ───────────────────────────────────────────────────
        elif op == 'ASSERT':
            if not V(a1): raise AssertionError()
        elif op == 'ASSERT_MSG': pass

        # ── Ternary（py0c 只帶 test 和 body，orelse 已丟失，回傳 body or None）─
        elif op == 'TERNARY':
            if V(a1):
                frame.set_local(r, V(a2))
            else:
                frame.set_local(r, None)

        # ── f-string / comprehension（簡化）────────────────────────
        elif op == 'FSTRING':
            frame.set_local(r, '')
        elif op in ('LIST_COMP', 'SET_COMP', 'DICT_COMP', 'GENERATOR', 'LAMBDA'):
            frame.set_local(r, None)

        # ── Type specialization ──────────────────────────────────────
        elif op in ('ASSUME_TYPE', 'BOX', 'UNBOX'):
            frame.set_local(r, V(a1))

        # ── 靜默忽略其餘結構性 / 未知指令 ───────────────────────────
        else:
            pass

        return None

    def _parse_key(self, key_str, frame):
        if key_str is None or key_str == '_': return None
        if ':' in key_str:
            parts = key_str.split(':')
            def p(s):
                if s == '_' or s == '': return None
                return frame.resolve(s)
            start = p(parts[0]) if len(parts) > 0 else None
            stop  = p(parts[1]) if len(parts) > 1 else None
            step  = p(parts[2]) if len(parts) > 2 else None
            return slice(start, stop, step)
        return frame.resolve(key_str)

# ─────────────────────────────────────────────
# 入口點  (py0i 風格：script 看到自己的 argv)
# ─────────────────────────────────────────────

def main():
    if len(sys.argv) < 2:
        print("用法: python qd0vm.py <file.qd> [args...]", file=sys.stderr)
        sys.exit(1)

    qd_path = sys.argv[1]
    script_argv = sys.argv[1:]   # 腳本看到 argv[0] = 自己的路徑

    import os
    if not os.path.exists(qd_path):
        print("qd0vm: can't open file '" + str(qd_path) + "'", file=sys.stderr)
        sys.exit(1)

    try:
        instructions, label_map, function_map, class_map = load_program(qd_path)
    except Exception as e:
        print("qd0vm: 載入失敗: " + str(e), file=sys.stderr)
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