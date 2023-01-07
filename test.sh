#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  ./target/release/main "$input" > tests/tmp/tmp.s
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

assert 0 "0;"
assert 42 "42;"
assert 21 "5+20-4;"
assert 41 " 12 + 34 - 5 ;"
assert 47 "5+6*7;"
assert 15 "5*(9-6);"
assert 4 "(3+5)/2;"
assert 12 "-( 3 + 5 ) + 20;"
assert 5 "-3*+5 + 20;"
assert 10 "- 10 + 20;"

assert 0 "0==1;"
assert 1 "42==42;"
assert 1 "0!=1;"
assert 0 "42!=42;"

assert 1 "0<1;"
assert 0 "1<1;"
assert 0 "2<1;"
assert 1 "0<=1;"
assert 1 "1<=1;"
assert 0 "2<=1;"

assert 1 "1>0;"
assert 0 "1>1;"
assert 0 "1>2;"
assert 1 "1>=0;"
assert 1 "1>=1;"
assert 0 "1>=2;"

# LOCAL VARIABLES
assert 3 "a=3; a;"
assert 8 "a=3; z=5; a+z;"
assert 6 "f=3 * 2; f;"
assert 21 "a=1; b=2; c=3; d=4; e=5; f=6; a+b+c+d+e+f;"

echo "===test end===;"
