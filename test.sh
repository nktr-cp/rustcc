#!/bin/bash

assert() {
  expected="$1"
  input="$2"

  cargo run -- "$input" >tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 "0;"
assert 42 "42;"

assert 21 "5+20-4;"

assert 41 " 12 + 34 - 5; "

assert 47 '5+6*7;'
assert 15 '5*(9-6);'
assert 4 '(3+5)/2;'

assert 10 "-10+20;"
assert 42 "(-5+11)*(-7/-1);"

assert 1 "1==1;"
assert 0 "1==2;"
assert 1 "1!=2;"
assert 0 "1!=1;"
assert 1 "1<2;"
assert 0 "1<1;"
assert 1 "1<=1;"
assert 1 "1<=2;"
assert 0 "1>2;"
assert 0 "1>1;"
assert 1 "2>1;"
assert 1 "2>=1;"
assert 1 "2>=2;"
assert 0 "1>=2;"

assert 42 "a=6; b=7; a*b;"
assert 14 "a=3; b=5*6-8; a+b/2;"
assert 1 "a=1; a==1;"
assert 42 "z=42; z;"

assert 42 "variable=42; variable;"
assert 42 "variable=42; another_variable=variable; another_variable;"
assert 42 "a=13; hoge=21; fuga=8; a=42; a;"

assert 42 "return 42;"
assert 42 "hello=42; world=12; return hello; world;"

echo OK
