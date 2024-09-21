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

assert 0 "
int x;

int main() {
	return x;
}
"

assert 42 "
int x;

int main() {
	x = 42;
	return x;
}

"

assert 42 "
int *x;

int main() {
	int y = 42;
	x = &y;
	return *x;
}
"

assert 42 "
int x[2];

int main() {
	x[1] = 42;
	return x[1];
}
"

assert 42 "
int x[2][3][4];

int main() {
	x[1][2][1] = 42;
	return x[1][2][1];
}

"

assert 42 "
int main() {
	int x[2][3];
	x[1][2] = 42;
	return x[1][2];
}
"

assert 42 "
int main() {
	int x[2][3][4];
	x[1][2][1] = 42;
	return x[1][2][1];
}
"

assert 42 "
int main() {
	int a[3][3][3][3];
	a[2][2][2][2] = 42;
	return a[2][2][2][2];
}
"

assert 55 "
int sumup(int m, int n) {
	int ret = 0;
	int i = 0;
	for (; i <= n; i = i + 1) {
		ret = ret + i;
	}
	return ret;
}

int main() {
	int a[2][2][2][2][2][2][2];
	a[0][1][0][1][0][1][0] = 1;
	a[1][0][1][0][1][0][1] = 10;
	return sumup(a[0][1][0][1][0][1][0], a[1][0][1][0][1][0][1]);
}
"

assert 42 "
int main() {
	int a[2];
	*a = 10;
	*(a + 1) = 22;
	int *p;
	p = a;
	*(p + 1) = 32;
	return *p + *(a + 1);
}
"

assert 42 "
int main() {
	int x = 3;
	int *y = &x;

	if (sizeof(3) != 4) return 1;
	if (sizeof(x) != 4) return 2;
	if (sizeof(y) != 8) return 3;

	if (sizeof(x + 3) != 4) return 4;
	if (sizeof(y + 3) != 8) return 5;
	if (sizeof(*y) != 4) return 6;

	if (sizeof(1) != 4) return 7;

	if (sizeof(sizeof(1)) != 4) return 8;

	return 42;
}
"

assert 42 "int f() {return 10;} int main() {return 42;}"

assert 55 "
int echo(int n) {
	return n;
}

int main() {
	return echo(55);
}"

assert 55 "
int sumup(int n) {
	if (n <= 1) {
		return n;
	}
	return n + sumup(n-1);
}

int main() {
	return sumup(10);
}
"

assert 55 "
int fib(int n) {
	if (n <= 1) {
		return n;
	}
	return fib(n-1) + fib(n-2);
}

int main() {
	return fib(10);
}
"

# forのinit部分はexprで処理しているので、代入は現状無理
assert 55 "
int sumup2(int m, int n) {
	int ret = 0;
	int i = 0;
	for (; i <= n; i = i + 1) {
		ret = ret + i;
	}
	return ret;
}

int main() {
	return sumup2(1, 10);
}
"

assert 42 "
int main() {
	int x = 21;
	int y = 42;
	return y;
}
"

assert 42 "
int main() {
	int x = 42;
	return *(&x);
}
"

assert 42 "
int main() {
	int x = 42;
	int *ptr = &x;
	return *ptr;
}
"

assert 42 "
int main() {
	int x = 42;
	int y = &x;
	int z = &y;
	return **z;
}
"

# ポインタに対する加減算を変更したのでテストも変更
# まだアラインメントが変なので2引かないといけない (要修正: issue #2)
assert 55 "
int main() {
	int x = 21;
	int y = 42;

	int* ptr = &x;
	ptr = ptr - 2;
	y = 55;
	return *ptr;
}
"

assert 0 "int main() {0;}"
assert 42 "int main() {42;}"

assert 21 "int main() {5+20-4;}"

assert 41 "int main() { 12 + 34 - 5; }"

assert 47 'int main() {5+6*7;}'
assert 15 'int main() {5*(9-6);}'
assert 4 'int main() {(3+5)/2;}'

assert 10 "int main() {-10+20;}"
assert 42 "int main() {(-5+11)*(-7/-1);}"

assert 1 "int main() {1==1;}"
assert 0 "int main() {1==2;}"
assert 1 "int main() {1!=2;}"
assert 0 "int main() {1!=1;}"
assert 1 "int main() {1<2;}"
assert 0 "int main() {1<1;}"
assert 1 "int main() {1<=1;}"
assert 1 "int main() {1<=2;}"
assert 0 "int main() {1>2;}"
assert 0 "int main() {1>1;}"
assert 1 "int main() {2>1;}"
assert 1 "int main() {2>=1;}"
assert 1 "int main() {2>=2;}"
assert 0 "int main() {1>=2;}"

assert 42 "int main() {int a=6; int b=7; a*b;}"
assert 14 "int main() {int a=3; int b=5*6-8; a+b/2;}"
assert 1 "int main() {int a=1; a==1;}"
assert 42 "int main() {int z=42; z;}"

assert 42 "int main() {int variable=42; variable;}"
assert 42 "int main() {int variable=42; int another_variable=variable; another_variable;}"
assert 42 "int main() {int a=13; int hoge=21; int fuga=8; a=42; a;}"

assert 42 "int main() {return 42;}"
assert 42 "int main() {int hello=42; int world=12; return hello; world;}"

assert 42 "int main() {int b=32; int a = 0; for(; a<10; a=a+1) b=b+1; b;}"
assert 42 "int main() {int b=37; int a = 0; for(; a<10; a=a+1) if(a==5) return a+b; return 255;}"

assert 42 "int main() {int a=42; if(a==42) return a; return 0;}"
assert 0 "int main() {int a=10; if(a==42) return 42; return 0;}"

assert 42 "int main() {int a=42; if(a==42) return 42; else return 0;}"
assert 0 "int main() {int a=10; if(a==42) return 42; else return 0;}"

assert 1 "int main() {int a=10; int b=20; if(a==10) if(b==20) return 1; else return 2; else return 3;}"
assert 3 "int main() {int a=30; int b=20; if(a==10) if(b==20) return 1; else return 2; else return 3;}"

assert 55 "int main() {int sum=0; int i=1; for(; i<=10; i=i+1) sum=sum+i; return sum;}"

assert 10 "int main() {int i=0; while(i<10) i=i+1; return i;}"

assert 42 "int main() {int a=0; while(a<42) if(a==30) a=a+2; else a=a+1; return a;}"
assert 41 "int main() {int a=0; while(a<40) a=a+1; if(a==40) a=a+1; return a;}"

assert 42 "int main() {int a=0; while(a<100) if(a==42) return a; else a=a+1; return 255;}"

assert 42 "
int main() {
	int a=0;
	while(a<100) {
		if (a==42) {
			return a;
		}
		a=a+1;
	}
	return 255;
}"

assert 42 "
int main() {
	int a=0;
	int i=0;
	for (i=0; i<42; i=i+1) {
		if (i==10) {
			a = a+2;
			i = i+1;
		} else {
			a = a+1;
		}
	}
	return a;
}
"

assert 42 "
int main() {
	int x = 12;
	int* y = &x;
	*y = 42;
	return x;
}
"

assert 42 "
int main() {
	int x = 12;
	int* y = &x;
	int** z = &y;
	int*** w = &z;
	***w = 42;
	return x;
}
"

fib() {
	expected="$1"
	input="$2"

	echo "int fibonacchi(int n) {
		if (n <= 1) {
			return n;
		}
		return fibonacchi(n-1) + fibonacchi(n-2);
	}" > fib.c

	cargo run -- "$input" > tmp.s
	cc -c fib.c
	cc -c tmp.s
	cc tmp.o fib.o -o tmp
	./tmp
	actual="$?"

	if [ "$actual" = "$expected" ]; then
		echo "$input => $actual"
	else
		echo "$input => $expected expected, but got $actual"
		exit 1
	fi
}

add() {
	expected="$1"
	input="$2"

	echo "int add(int a, int b) {
		return a+b;
	}" > add.c

	cargo run -- "$input" > tmp.s
	cc -c add.c
	cc -c tmp.s
	cc tmp.o add.o -o tmp
	./tmp
	actual="$?"

	if [ "$actual" = "$expected" ]; then
		echo "$input => $actual"
	else
		echo "$input => $expected expected, but got $actual"
		exit 1
	fi
}

no_arg() {
	expected="$1"
	input="$2"

	echo "int no_arg() {
		return $expected;
	}" > no_arg.c

	cargo run -- "$input" > tmp.s
	cc -c no_arg.c
	cc -c tmp.s
	cc tmp.o no_arg.o -o tmp
	./tmp
	actual="$?"

	if [ "$actual" = "$expected" ]; then
		echo "$input => $actual"
	else
		echo "$input => $expected expected, but got $actual"
		exit 1
	fi
}

alloc4() {
	expected="$1"
	input="$2"

	echo "
	#include <stdlib.h>
	void alloc4(int** ptr, int a, int b, int c, int d) {
		*ptr = (int*)malloc(4 * sizeof(int));
		**ptr = a;
		*(*ptr + 1) = b;
		*(*ptr + 2) = c;
		*(*ptr + 3) = d;
	}" > alloc4.c

	cargo run -- "$input" > tmp.s
	cc -c alloc4.c
	cc -c tmp.s
	cc tmp.o alloc4.o -o tmp
	./tmp
	actual="$?"

	if [ "$actual" = "$expected" ]; then
		echo "$input => $actual"
	else
		echo "$input => $expected expected, but got $actual"
		exit 1
	fi
}


no_arg 42 "int main() {return no_arg();}"
fib 55 "int main() {return fibonacchi(10);}"
add 42 "int main() {add(20, 22);}"
alloc4 42 "
int main() {
	int *p;
	alloc4(&p, 0, 0, 0, 42);
	int *q;
	q = p + 3;
	return *q;
}"


rm -f tmp* *.s *.c *.o 

echo OK
