#include "ast.h"
#include "eval.h"
#include "runtime.h"
#include "util.h"

#include <stdio.h>
#include <stdlib.h>

static char *read_file(const char *path) {
    FILE *fp = fopen(path, "rb");
    if (!fp) {
        die("cannot open %s", path);
    }
    fseek(fp, 0, SEEK_END);
    long size = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    char *buf = (char *)malloc((size_t)size + 1);
    if (!buf) {
        die("out of memory");
    }
    if (fread(buf, 1, (size_t)size, fp) != (size_t)size) {
        die("failed to read %s", path);
    }
    buf[size] = '\0';
    fclose(fp);
    return buf;
}

int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "Usage: ./py0i <script.py>\n");
        return 1;
    }

    char *source = read_file(argv[1]);
    AstNode *module = parse_source(source, argv[1]);
    PyRuntime *rt = runtime_new();
    ExecResult result = exec_module(rt, rt->globals, module);
    (void)result;
    free(source);
    ast_free(module);
    return 0;
}
