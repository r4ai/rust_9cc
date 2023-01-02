#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  ./target/debug/main "$input" > tests/tmp/tmp.s
  cc -o tests/tmp/tmp.out tests/tmp/tmp.s
  ./tests/tmp/tmp.out
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
  fi
}

cargo build -r
mkdir ./tests/tmp

echo "===test start==="

assert 0 0
assert 42 42
assert 21 "5+20-4"
assert 41 " 12 + 34 - 5 "
assert 47 "5+6*7"
assert 15 "5*(9-6)"
assert 4 "(3+5)/2"

echo "===test end==="
