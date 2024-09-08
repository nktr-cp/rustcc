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

assert 42 "b=32; for(a=0; a<10; a=a+1) b=b+1; b;"
assert 42 "b=37; for(a=0; a<10; a=a+1) if(a==5) return a+b; return 255;"

assert 42 "a=42; if(a==42) return a; return 0;"
assert 0 "a=10; if(a==42) return 42; return 0;"

assert 42 "a=42; if(a==42) return 42; else return 0;"
assert 0 "a=10; if(a==42) return 42; else return 0;"

assert 1 "a=10; b=20; if(a==10) if(b==20) return 1; else return 2; else return 3;"
assert 3 "a=30; b=20; if(a==10) if(b==20) return 1; else return 2; else return 3;"

assert 55 "sum=0; for(i=1; i<=10; i=i+1) sum=sum+i; return sum;"

assert 10 "i=0; while(i<10) i=i+1; return i;"

assert 42 "a=0; while(a<42) if(a==30) a=a+2; else a=a+1; return a;"
assert 41 "a=0; while(a<40) a=a+1; if(a==40) a=a+1; return a;"

assert 42 "a=0; while(a<100) if(a==42) return a; else a=a+1; return 255;"

echo OK
