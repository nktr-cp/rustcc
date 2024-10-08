# simple C compiler written in Rust

## Usage
```bash
$ git clone https://github.com/nktr-cp/rustcc
$ cd rustcc

$ cargo make image # イメージのビルド
$ cargo make login # コンテナ内にログイン
$ cargo make cb_test # test.shのテストをコンテナ内で実行

$ cargo run $SOURCE_CODE # 引数にソースコードを渡してコード生成
```

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
              | string_literal
arglist     ::= expr ("," expr)*
type        ::= base_type "*"*
base_type   ::= "int" | "char"
```

### note
- typeの中身はnumにする、exprは対応しない様にする (const変数に対応するなら必要かも)

## Acknowledgments
- [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook)
