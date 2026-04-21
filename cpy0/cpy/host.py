import sys


def exec_with_py_frontend(path, script_argv):
    with open(path, "r", encoding="utf-8") as f:
        source = f.read()

    globals_dict = {
        "__name__": "__main__",
        "__file__": path,
        "__builtins__": __builtins__,
        "sys": sys,
        # False-y sentinel: lets cpy0i.py know it is still in the Python stage.
        "run_path": "",
    }

    old_argv = sys.argv
    sys.argv = list(script_argv)
    try:
        exec(compile(source, path, "exec"), globals_dict, globals_dict)
    finally:
        sys.argv = old_argv


def main():
    if len(sys.argv) < 2:
        print("Usage: python host.py <script.py> [args...]")
        raise SystemExit(1)

    target = sys.argv[1]
    exec_with_py_frontend(target, sys.argv[1:])


if __name__ == "__main__":
    main()
