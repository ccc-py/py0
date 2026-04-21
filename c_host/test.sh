#!/usr/bin/env bash

set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
BIN="$ROOT/py0i"
TEST_ROOT="$(cd "$ROOT/../tests" && pwd)"
PY_HOST_ROOT="$(cd "$ROOT/../py_host" && pwd)"

if [[ ! -x "$BIN" ]]; then
  echo "py0i binary not found, building first..."
  make -C "$ROOT"
fi

run_test() {
  local name="$1"
  local expected="$2"
  local script="$TEST_ROOT/$name.py"

  echo "[TEST] $name"
  local actual
  actual="$("$BIN" "$script")"

  if [[ "$actual" != "$expected" ]]; then
    echo "  FAIL"
    echo "  expected:"
    printf '<<EOF\n%s\nEOF\n' "$expected"
    echo "  actual:"
    printf '<<EOF\n%s\nEOF\n' "$actual"
    exit 1
  fi

  echo "  PASS"
}

run_test "hello" "hello"
run_test "vars" "3"
run_test "ifwhile" $'3\n2\n1'
run_test "func" "5"
run_test "fact" "120"

echo "[TEST] c-self-hello"
c_self_hello="$("$BIN" "$PY_HOST_ROOT/cpy0i.py" "$TEST_ROOT/hello.py")"
if [[ "$c_self_hello" != "hello" ]]; then
  echo "  FAIL"
  printf 'expected: <<EOF\nhello\nEOF\n'
  printf 'actual: <<EOF\n%s\nEOF\n' "$c_self_hello"
  exit 1
fi
echo "  PASS"

echo "[TEST] c-self-fact"
c_self_fact="$("$BIN" "$PY_HOST_ROOT/cpy0i.py" "$PY_HOST_ROOT/cpy0i.py" "$TEST_ROOT/fact.py")"
if [[ "$c_self_fact" != "120" ]]; then
  echo "  FAIL"
  printf 'expected: <<EOF\n120\nEOF\n'
  printf 'actual: <<EOF\n%s\nEOF\n' "$c_self_fact"
  exit 1
fi
echo "  PASS"

echo
echo "All tests passed."
