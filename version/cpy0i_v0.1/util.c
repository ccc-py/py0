#include "util.h"

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void ptrvec_init(PtrVec *vec) {
    vec->items = NULL;
    vec->count = 0;
    vec->capacity = 0;
}

void ptrvec_push(PtrVec *vec, void *item) {
    if (vec->count == vec->capacity) {
        int new_cap = vec->capacity ? vec->capacity * 2 : 8;
        void **new_items = (void **)realloc(vec->items, sizeof(void *) * new_cap);
        if (!new_items) {
            die("out of memory");
        }
        vec->items = new_items;
        vec->capacity = new_cap;
    }
    vec->items[vec->count++] = item;
}

void ptrvec_free(PtrVec *vec) {
    free(vec->items);
    vec->items = NULL;
    vec->count = 0;
    vec->capacity = 0;
}

char *xstrdup(const char *src) {
    size_t len = strlen(src);
    char *out = (char *)malloc(len + 1);
    if (!out) {
        die("out of memory");
    }
    memcpy(out, src, len + 1);
    return out;
}

char *xstrndup(const char *src, size_t n) {
    char *out = (char *)malloc(n + 1);
    if (!out) {
        die("out of memory");
    }
    memcpy(out, src, n);
    out[n] = '\0';
    return out;
}

void strbuf_init(StrBuf *buf) {
    buf->data = NULL;
    buf->len = 0;
    buf->cap = 0;
}

static void strbuf_reserve(StrBuf *buf, size_t extra) {
    size_t need = buf->len + extra + 1;
    if (need <= buf->cap) {
        return;
    }
    size_t new_cap = buf->cap ? buf->cap * 2 : 32;
    while (new_cap < need) {
        new_cap *= 2;
    }
    char *new_data = (char *)realloc(buf->data, new_cap);
    if (!new_data) {
        die("out of memory");
    }
    buf->data = new_data;
    buf->cap = new_cap;
}

void strbuf_append_char(StrBuf *buf, char ch) {
    strbuf_reserve(buf, 1);
    buf->data[buf->len++] = ch;
    buf->data[buf->len] = '\0';
}

void strbuf_append_str(StrBuf *buf, const char *s) {
    size_t n = strlen(s);
    strbuf_reserve(buf, n);
    memcpy(buf->data + buf->len, s, n);
    buf->len += n;
    buf->data[buf->len] = '\0';
}

char *strbuf_take(StrBuf *buf) {
    if (!buf->data) {
        return xstrdup("");
    }
    char *out = buf->data;
    buf->data = NULL;
    buf->len = 0;
    buf->cap = 0;
    return out;
}

void die(const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    fputc('\n', stderr);
    exit(1);
}
