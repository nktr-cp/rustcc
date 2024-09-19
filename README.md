# simple C compiler written in Rust

## Production rule
生成規則:
```
program    ::= function*
function   ::= type ident "(" paramlist? ")" "{" stmt* "}"
paramlist  ::= type ident ("," type ident)*
stmt       ::= expr ";"
             | "{" stmt* "}"
             | "if" "(" expr ")" stmt ("else" stmt)?
             | "while" "(" expr ")" stmt
             | "for" "(" expr? ";" expr? ";" expr? ")" stmt
             | "return" expr ";"
             | decl ";"
decl       ::= type ident ("[" expr "]") ("=" init)?
init       ::= expr
             | "{" expr? ("," expr)* "}"
expr       ::= assign
assign     ::= equlatity ("=" assign)?
equality   ::= relational ("==" relational | "!=" relational)*
relational ::= add ("<" add | "<=" add | ">" add | ">=" add)*
add        ::= mul ("+" mul | "-" mul)*
mul        ::= unary ("*" unary | "/" unary)*
unary      ::= "sizeof" unary
             | ("+" | "-")? primary
             | ("*" | "&") unary
primary    ::= num
             | ident ("(" arglist? ")")?
             | ident "[" expr "]"
             | "(" expr ")"
arglist    ::= type ("," expr)*
type       ::= base_type ("*" | "[" expr "]")*
base_type  ::= "int"
```
note: 現状配列サイズの暗黙な指定はサポートしていない

## Acknowledgments
- [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook)
