# AGENTS.md - py0 Project

## Commands

### py_full (Python interpreter)
```bash
cd /Users/Shared/ccc/project/py0
python py_full/py0i.py <script.py> [args...]
```

### py0c (Python → QD compiler)
```bash
cd /Users/Shared/ccc/project/py0/py0c
python py0c.py <script.py> -o qd/output.qd
python qd0vm.py qd/output.qd          # Python VM
gcc qd0vm.c -o qd0vm && ./qd0vm qd/output.qd  # C VM
```

### Tests (run from py0c/ directory)
```bash
cd /Users/Shared/ccc/project/py0/py0c
bash pytest.sh   # Python VM tests
bash ctest.sh    # C VM tests
```

## Structure

- `py_full/` - Python interpreter using Python stdlib `ast` module
- `py0c/` - Python → QD IR compiler + QD VM (Python + C implementations)
- `_doc/qd0spec.md` - QD quadruple IR specification
- `py0c/py/` - test scripts (hello.py, fact.py, json.py, class.py, oop.py)
- `py0c/qd/` - compiled QD output files

## Critical implementation details

- **Local module imports**: py0c uses local `ast.py` and `operator.py` via `sys.path.insert(0, '...')` at py0c/py0c.py:11-12, NOT stdlib
- **QD IR format**: 4-field text IR (op arg1 arg2 result), whitespace/comma separated
- **Python VM**: py0c/qd0vm.py is self-compilable (can compile itself)
- **C VM**: py0c/qd0vm.c is the C implementation

## Known working targets (Python VM)

```
hello.py  - basic operations ✓
fact.py  - recursion ✓
json.py  - dict/list/f-string operations ✓
class.py - class definition + methods ✓
oop.py   - class instantiation + OOP ✓
qd0vm.py - compiler self-hosting (fails - documented)
```

## Current issues

- **CLASS support**: Class compilation works, instance creation works, but attribute/method access has bugs (qd0vm.py self-hosting fails)
- **qd0vm.py VM bootstrap**: Running qd0vm.qd on qd0vm.py fails - qd0vm.qd cannot execute itself

## Entry points

- py_full: `main()` at py_full/py0i.py:992, `sys.argv[1]` = script path
- py0c: `main()` at py0c/py0c.py:761, accepts `-o output.qd` flag
- qd0vm (Python): `main()` at py0c/qd0vm.py:670
