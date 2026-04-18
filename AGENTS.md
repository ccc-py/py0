# AGENTS.md - py0 Project

## Commands

### py0i (Python interpreter)
```bash
python py0i/py0i.py <script.py> [args...]
```

### py0c (Python → QD compiler)
```bash
python py0c/py0c.py <script.py> -o output.qd
python py0c/qd0vm.py output.qd          # Python VM
gcc py0c/qd0vm.c -o py0c/qd0vm && ./py0c/qd0vm output.qd  # C VM
```

### Tests
```bash
bash py0c/pytest.sh   # Python VM tests
bash py0c/ctest.sh    # C VM tests
```

## Structure

- `py0i/` - Python interpreter (uses local ast.py, not stdlib)
- `py0c/` - Python → QD IR compiler + QD VM (Python + C)
- `_doc/qd0spec.md` - QD quadruple IR specification

## Important notes

- py0c uses local `ast.py` and `operator.py` via `sys.path.insert(0, ...)` at line 11-12
- QD format: 4-field text IR (op arg1 arg2 result)
- Target files compile: `py/hello.py`, `py/fact.py`, `py/json.py`, `qd0vm.py`

## Entry points

- py0i: `main()` at py0i/py0i.py:992, expects script path in `sys.argv[1]`
- py0c: `main()` at py0c/py0c.py:675, accepts `-o output.qd` flag