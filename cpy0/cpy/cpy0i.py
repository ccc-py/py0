sys = __import__("sys")


def c_bin_path():
    return __file__.replace("/cpy/cpy0i.py", "/c/py0i")


def call_c():
    os = __import__("os")
    cmd = c_bin_path() + " " + __file__

    if len(sys.argv) > 1:
        cmd = cmd + " " + sys.argv[1]
    if len(sys.argv) > 2:
        cmd = cmd + " " + sys.argv[2]
    if len(sys.argv) > 3:
        cmd = cmd + " " + sys.argv[3]

    return os.system(cmd)


def main():
    if len(sys.argv) < 2:
        print("Usage: cpy0i.py <script.py> [args...]")
        return

    if run_path:
        run_path(sys.argv[1])
        return

    call_c()


if __name__ == "__main__":
    main()
