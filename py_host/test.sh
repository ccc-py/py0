#!/usr/bin/env bash

set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
BIN="python3 $ROOT/host.py $ROOT/cpy0i.py"
TEST_ROOT="$(cd "$ROOT/../tests" && pwd)"

run_test() {
  local name="$1"
  local expected="$2"
  local script="$TEST_ROOT/$name.py"

  echo "[TEST] $name"
  local actual
  actual="$(python3 "$ROOT/host.py" "$ROOT/cpy0i.py" "$script")"

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

echo "[TEST] self-hello"
self_hello="$(python3 "$ROOT/host.py" "$ROOT/cpy0i.py" "$ROOT/cpy0i.py" "$TEST_ROOT/hello.py")"
if [[ "$self_hello" != "hello" ]]; then
  echo "  FAIL"
  printf 'expected: <<EOF\nhello\nEOF\n'
  printf 'actual: <<EOF\n%s\nEOF\n' "$self_hello"
  exit 1
fi
echo "  PASS"

echo "[TEST] self-fact"
self_fact="$(python3 "$ROOT/host.py" "$ROOT/cpy0i.py" "$ROOT/cpy0i.py" "$TEST_ROOT/fact.py")"
if [[ "$self_fact" != "120" ]]; then
  echo "  FAIL"
  printf 'expected: <<EOF\n120\nEOF\n'
  printf 'actual: <<EOF\n%s\nEOF\n' "$self_fact"
  exit 1
fi
echo "  PASS"

echo
echo "All tests passed."
