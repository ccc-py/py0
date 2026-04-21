#!/usr/bin/env bash

set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
BIN="$ROOT/py0i"

if [[ ! -x "$BIN" ]]; then
  echo "py0i binary not found, building first..."
  make -C "$ROOT"
fi

run_test() {
  local name="$1"
  local expected="$2"
  local script="$ROOT/tests/$name.py"

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

echo
echo "All tests passed."
