#ifndef CPY0I_EXCEPTION_H
#define CPY0I_EXCEPTION_H

typedef struct {
    int has_return;
    struct PyValue *value;
} ExecResult;

#endif
