#!/usr/bin/env bash

set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$ROOT/../.." && pwd)"
TEST_ROOT="$REPO_ROOT/tests"
C_ROOT="$REPO_ROOT/cpy0/c"

if [[ ! -x "$C_ROOT/py0i" ]]; then
  echo "c host binary not found, building first..."
  make -C "$C_ROOT"
fi

cd "$REPO_ROOT"

run_test() {
  local name="$1"
  local expected="$2"
  local actual

  echo "[TEST] $name"
  actual="$(python3 cpy0/cpy/host.py cpy0/cpy/cpy0i.py "tests/$name.py")"

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
run_test "fact" "120"

echo "[TEST] py+c-self-hello"
self_hello="$(python3 cpy0/cpy/host.py cpy0/cpy/cpy0i.py cpy0/cpy/cpy0i.py tests/hello.py)"
if [[ "$self_hello" != "hello" ]]; then
  echo "  FAIL"
  printf 'expected: <<EOF\nhello\nEOF\n'
  printf 'actual: <<EOF\n%s\nEOF\n' "$self_hello"
  exit 1
fi
echo "  PASS"

echo "[TEST] py+c-self-fact"
self_fact="$(python3 cpy0/cpy/host.py cpy0/cpy/cpy0i.py cpy0/cpy/cpy0i.py tests/fact.py)"
if [[ "$self_fact" != "120" ]]; then
  echo "  FAIL"
  printf 'expected: <<EOF\n120\nEOF\n'
  printf 'actual: <<EOF\n%s\nEOF\n' "$self_fact"
  exit 1
fi
echo "  PASS"

echo
echo "All tests passed."
