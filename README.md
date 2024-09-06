# simple C compiler written in Rust

## Production rule
比較演算子をサポートした文法
```
expr       ::= equality
equality   ::= relational ("==" relational | "!=" relational)*
relational ::= add ("<" add | "<=" add | ">" add | ">=" add)*
add        ::= mul ("+" mul | "-" mul)*
mul        ::= unary ("*" unary | "/" unary)*
unary      ::= ("+" | "-")? primary
primary    ::= num | "(" expr ")"
```

## Acknowledgments
- [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook)
