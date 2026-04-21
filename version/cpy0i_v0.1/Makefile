CC = gcc
CFLAGS = -std=c11 -Wall -Wextra -O0 -g

OBJS = \
	py0i.o runtime.o value.o env.o ast.o ast_nodes.o lexer.o parser.o \
	eval.o builtin.o operator.o util.o call.o function.o \
	class.o exception.o stmt.o expr.o module.o import.o \
	sysmod.o osmod.o iomod.o tracebackmod.o type.o object.o memory.o \
	string.o list.o tuple.o dict.o slice.o iter.o

py0i: $(OBJS)
	$(CC) $(CFLAGS) -o $@ $(OBJS)

test: py0i
	bash ./test.sh

clean:
	rm -f $(OBJS) py0i
