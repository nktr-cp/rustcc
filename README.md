# simple C compiler written in Rust

## Production rule
四則演算の文法
```
expr    ::= mul ("+" mul | "-" mul)*
mul     ::= unary ("*" unrary | "/" unary)*
unary   ::= ("+" | "-")? primary
primary ::= num | "(" expr ")"
```

## Acknowledgments
- [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook)
