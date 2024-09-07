# simple C compiler written in Rust

## Production rule
生成規則:
```
program    ::= stmt*
stmt       ::= expr ";" | "return" expr ";"
expr       ::= assign
assign     ::= equlatity ("=" assign)?
equality   ::= relational ("==" relational | "!=" relational)*
relational ::= add ("<" add | "<=" add | ">" add | ">=" add)*
add        ::= mul ("+" mul | "-" mul)*
mul        ::= unary ("*" unary | "/" unary)*
unary      ::= ("+" | "-")? primary
primary    ::= num | ident | "(" expr ")"
```

## Acknowledgments
- [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook)
