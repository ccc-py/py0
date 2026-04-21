#ifndef CPY0I_UTIL_H
#define CPY0I_UTIL_H

#include <stddef.h>

typedef struct {
    void **items;
    int count;
    int capacity;
} PtrVec;

typedef struct {
    char *data;
    size_t len;
    size_t cap;
} StrBuf;

void ptrvec_init(PtrVec *vec);
void ptrvec_push(PtrVec *vec, void *item);
void ptrvec_free(PtrVec *vec);

char *xstrdup(const char *src);
char *xstrndup(const char *src, size_t n);
void strbuf_init(StrBuf *buf);
void strbuf_append_char(StrBuf *buf, char ch);
void strbuf_append_str(StrBuf *buf, const char *s);
char *strbuf_take(StrBuf *buf);
void die(const char *fmt, ...);

#endif
