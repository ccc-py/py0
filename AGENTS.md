# AGENTS.md - py0 Project

## Commands

### py0 (Python interpreter)
```bash
cd /Users/Shared/ccc/project/py0
python py0/py0i.py <script.py> [args...]
```

### cpy0 Python host
```bash
cd /Users/Shared/ccc/project/py0
python cpy0/py/host.py cpy0/py/cpy0i.py <script.py> [args...]
```

### cpy0 C host
```bash
cd /Users/Shared/ccc/project/py0/cpy0/c
./py0i ../../tests/hello.py
```

## Structure

- `py0/` - Python interpreter using Python stdlib `ast` module
- `cpy0/py/` - minimal Python host and self-host wrapper
- `cpy0/c/` - minimal C host and runtime

## Entry points

- py0: `main()` at py0/py0i.py:992, `sys.argv[1]` = script path
