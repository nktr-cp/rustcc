# simple C compiler written in Rust

## Production rule
生成規則:
```
program     ::= (type ident (function | global_decl))*
function    ::= "(" paramlist? ")" "{" stmt* "}"
global_decl ::= ("[" num "]")*
paramlist   ::= type ident ("," type ident)*
stmt        ::= expr ";"
              | "{" stmt* "}"
              | "if" "(" expr ")" stmt ("else" stmt)?
              | "while" "(" expr ")" stmt
              | "for" "(" expr? ";" expr? ";" expr? ")" stmt
              | "return" expr ";"
              | decl ";"
decl        ::= type ident ("[" num "]")* ("=" expr)?
expr        ::= assign
assign      ::= equlatity ("=" assign)?
equality    ::= relational ("==" relational | "!=" relational)*
relational  ::= add ("<" add | "<=" add | ">" add | ">=" add)*
add         ::= mul ("+" mul | "-" mul)*
mul         ::= unary ("*" unary | "/" unary)*
unary       ::= "sizeof" unary
              | ("+" | "-")? primary
              | ("*" | "&") unary
primary     ::= num
              | ident ("(" arglist? ")")?
              | ident ("[" expr "]")+
              | "(" expr ")"
arglist     ::= expr ("," expr)*
type        ::= base_type "*"*
base_type   ::= "int"
```

### note
- typeの中身はnumにする、exprは対応しない様にする (const変数に対応するなら必要かも)

## Acknowledgments
- [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook)
