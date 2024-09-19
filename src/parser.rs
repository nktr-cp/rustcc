use crate::lexer::{Token, TokenKind};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOpKind {
    Ref,
    Deref,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOpKind {
    Eq,
    Nq,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Int,
    Ptr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub kind: TypeKind,
    pub ptr_to: Option<Box<Type>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LVar {
    pub name: String,
    pub offset: usize,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    BinaryOp(BinaryOpKind),                // Binaty operations: +, -, *, /
    UnaryOp(UnaryOpKind),                  // Unary operations: &, *
    Comparison(ComparisonOpKind),          // Comparison operations: ==, !=, <, <=, >, >=
    Num(i32),                              // Numeric literals
    LVar(LVar),                            // Local variable
    Assign,                                // Assignment
    Return,                                // Return statement
    Block(Vec<Node>),                      // Block of statements
    Fncall(Function, Vec<Node>),           // Function call with arguments
    Fndef(Function, Vec<Node>, Vec<LVar>), // Function definition (name, parameters, local variables)
    For,                                   // For
    While,                                 // While
    If,                                    // If
    Else,                                  // Else
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub ty: Type,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    fn_idx: usize,
    pub locals: Vec<HashMap<String, LVar>>, // function name -> local variables
    pub functions: Vec<Function>,
}

// 左右の子に応じてノードの型を決定する
fn create_new_node(kind: NodeKind, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Node {
    let ty = match &kind {
        NodeKind::BinaryOp(op) => match op {
            BinaryOpKind::Add | BinaryOpKind::Sub => {
                if let (Some(l), Some(r)) = (&lhs, &rhs) {
                    if l.ty.kind == TypeKind::Ptr && r.ty.kind == TypeKind::Int {
                        l.ty.clone()
                    } else if l.ty.kind == TypeKind::Int && r.ty.kind == TypeKind::Ptr {
                        r.ty.clone()
                    } else {
                        Type {
                            kind: TypeKind::Int,
                            ptr_to: None,
                        }
                    }
                } else {
                    Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    }
                }
            }
            _ => Type {
                kind: TypeKind::Int,
                ptr_to: None,
            },
        },
        NodeKind::UnaryOp(op) => match op {
            UnaryOpKind::Ref => {
                if let Some(l) = &lhs {
                    Type {
                        kind: TypeKind::Ptr,
                        ptr_to: Some(Box::new(l.ty.clone())),
                    }
                } else {
                    Type {
                        kind: TypeKind::Ptr,
                        ptr_to: Some(Box::new(Type {
                            kind: TypeKind::Int,
                            ptr_to: None,
                        })),
                    }
                }
            }
            UnaryOpKind::Deref => {
                if let Some(l) = &lhs {
                    if let Some(ptr_to) = &l.ty.ptr_to {
                        (**ptr_to).clone()
                    } else {
                        Type {
                            kind: TypeKind::Int,
                            ptr_to: None,
                        }
                    }
                } else {
                    Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    }
                }
            }
        },
        NodeKind::Comparison(_) => Type {
            kind: TypeKind::Int,
            ptr_to: None,
        },
        NodeKind::Num(_) => Type {
            kind: TypeKind::Int,
            ptr_to: None,
        },
        NodeKind::LVar(ref lvar) => lvar.ty.clone(),
        NodeKind::Assign => {
            if let Some(l) = &lhs {
                l.ty.clone()
            } else {
                Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                }
            }
        }
        NodeKind::Return => Type {
            kind: TypeKind::Int,
            ptr_to: None,
        },
        NodeKind::Block(_) => Type {
            kind: TypeKind::Int,
            ptr_to: None,
        },
        NodeKind::Fncall(ref func, _) => func.ty.clone(),
        NodeKind::Fndef(ref func, _, _) => func.ty.clone(),
        NodeKind::For | NodeKind::While | NodeKind::If | NodeKind::Else => Type {
            kind: TypeKind::Int,
            ptr_to: None,
        },
    };

    Node { kind, lhs, rhs, ty }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            fn_idx: 0, // indicates the index of the current function
            locals: Vec::new(),
            functions: Vec::new(),
        }
    }

    fn create_lvar(&mut self, name: &str, ty: Type) -> LVar {
        if let Some(lvar) = self.locals[self.fn_idx].get(name) {
            return lvar.clone();
        }
        let lvar = LVar {
            name: name.to_string(),
            offset: self.locals[self.fn_idx].len() * 8 + 8,
            ty: ty.clone(),
        };
        self.locals[self.fn_idx].insert(name.to_string(), lvar.clone());
        lvar
    }

    fn find_lvar(&self, name: &str) -> Option<&LVar> {
        self.locals[self.fn_idx].get(name)
    }

    fn consume(&mut self, op: &str) -> bool {
        if self.tokens[self.pos].str != op.to_string() {
            return false;
        }
        self.pos += 1;
        true
    }

    fn expect(&mut self, op: &str) -> Result<(), String> {
        if !self.consume(op) {
            return Err(format!(
                "'{}'が期待されますが、'{}'でした",
                op, self.tokens[self.pos].str
            ));
        }
        Ok(())
    }

    fn expect_number(&mut self) -> Result<i32, String> {
        if self.tokens[self.pos].kind != TokenKind::Num {
            return Err(format!(
                "数が期待されますが、'{}'でした",
                self.tokens[self.pos].str
            ));
        }
        let val = self.tokens[self.pos].val.unwrap();
        self.pos += 1;
        Ok(val)
    }

    fn at_eof(&self) -> bool {
        self.tokens[self.pos].kind == TokenKind::Eof
    }

    pub fn program(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        while !self.at_eof() {
            self.locals.push(HashMap::new());
            nodes.push(self.function()?);
            self.fn_idx += 1;
        }
        return Ok(nodes);
    }

    fn function(&mut self) -> Result<Node, String> {
        let ty = self.ty()?;
        let name = self.tokens[self.pos].str.clone();
        self.pos += 1;
        self.expect("(")?;

        let mut params = Vec::new();
        if !self.consume(")") {
            params = self.paramlist()?;
            self.expect(")")?;
        }

        let func = Function {
            name: name.clone(),
            ty: ty.clone(),
        };

        if self.functions.contains(&func) {
            return Err(format!("関数{}は既に定義されています", name));
        } else {
            self.functions.push(func.clone());
        }

        let body = self.stmt()?;

        let rhs = Some(Box::new(body));
        Ok(create_new_node(
            NodeKind::Fndef(
                func.clone(),
                params.clone(),
                self.locals[self.fn_idx].values().cloned().collect(),
            ),
            None,
            rhs,
        ))
    }

    fn paramlist(&mut self) -> Result<Vec<Node>, String> {
        let mut params = Vec::new();

        loop {
            let ty = self.ty()?;
            let lvar = self.create_lvar(&self.tokens[self.pos].str.clone(), ty.clone());
            params.push(create_new_node(NodeKind::LVar(lvar), None, None));
            self.pos += 1;
            if !self.consume(",") {
                break;
            }
        }
        Ok(params)
    }

    fn stmt(&mut self) -> Result<Node, String> {
        let node: Node;
        if self.consume("{") {
            let mut stmts = Vec::new();
            loop {
                if self.consume("}") {
                    break;
                }
                stmts.push(self.stmt()?);
            }
            node = create_new_node(NodeKind::Block(stmts), None, None);
        } else if self.consume("return") {
            node = create_new_node(NodeKind::Return, Some(Box::new(self.expr()?)), None);
            self.expect(";")?;
        } else if self.consume("for") {
            let mut init = create_new_node(NodeKind::Num(0), None, None);
            let mut cond = create_new_node(NodeKind::Num(1), None, None);
            let mut inc = create_new_node(NodeKind::Num(0), None, None);
            self.expect("(")?;
            if !self.consume(";") {
                init = self.expr()?;
                self.expect(";")?;
            }
            if !self.consume(";") {
                cond = self.expr()?;
                self.expect(";")?;
            }
            if !self.consume(")") {
                inc = self.expr()?;
                self.expect(")")?;
            }
            let lhs = Some(Box::new(init));
            let rrhs = create_new_node(
                NodeKind::For,
                Some(Box::new(inc)),
                Some(Box::new(self.stmt()?)),
            );
            let rhs = create_new_node(NodeKind::For, Some(Box::new(cond)), Some(Box::new(rrhs)));
            node = create_new_node(NodeKind::For, lhs, Some(Box::new(rhs)));
        } else if self.consume("while") {
            self.expect("(")?;
            let cond = self.expr()?;
            self.expect(")")?;
            let rhs = Some(Box::new(self.stmt()?));
            node = create_new_node(NodeKind::While, Some(Box::new(cond)), rhs);
        } else if self.consume("if") {
            self.expect("(")?;
            let cond = self.expr()?;
            self.expect(")")?;
            let then = self.stmt()?;
            if self.consume("else") {
                let els = self.stmt()?;
                let rhs =
                    create_new_node(NodeKind::Else, Some(Box::new(then)), Some(Box::new(els)));
                node = create_new_node(NodeKind::If, Some(Box::new(cond)), Some(Box::new(rhs)));
            } else {
                node = create_new_node(NodeKind::If, Some(Box::new(cond)), Some(Box::new(then)));
            }
        } else if self.tokens[self.pos].str == "int" {
            node = self.decl()?;
            self.expect(";")?;
        } else {
            node = self.expr()?;
            self.expect(";")?;
        }
        Ok(node)
    }

    fn decl(&mut self) -> Result<Node, String> {
        let node;
        let ty = self.ty()?;
        let lvar = self.create_lvar(&self.tokens[self.pos].str.clone(), ty);
        self.pos += 1;
        if self.consume("=") {
            let lhs = create_new_node(NodeKind::LVar(lvar.clone()), None, None);
            node = create_new_node(
                NodeKind::Assign,
                Some(Box::new(lhs)),
                Some(Box::new(self.expr()?)),
            );
        } else {
            node = create_new_node(NodeKind::LVar(lvar.clone()), None, None);
        }
        Ok(node)
    }

    fn expr(&mut self) -> Result<Node, String> {
        return self.assign();
    }

    fn assign(&mut self) -> Result<Node, String> {
        let mut node = self.equality()?;

        if self.consume("=") {
            node = create_new_node(
                NodeKind::Assign,
                Some(Box::new(node)),
                Some(Box::new(self.assign()?)),
            );
        }

        return Ok(node);
    }

    fn equality(&mut self) -> Result<Node, String> {
        let mut node = self.relational()?;

        loop {
            if self.consume("==") {
                let lhs = Some(Box::new(node));
                let rhs = Some(Box::new(self.relational()?));
                node = create_new_node(NodeKind::Comparison(ComparisonOpKind::Eq), lhs, rhs);
            } else if self.consume("!=") {
                let lhs = Some(Box::new(node));
                let rhs = Some(Box::new(self.relational()?));
                node = create_new_node(NodeKind::Comparison(ComparisonOpKind::Nq), lhs, rhs);
            } else {
                return Ok(node);
            }
        }
    }

    fn relational(&mut self) -> Result<Node, String> {
        let mut node = self.add()?;

        loop {
            if self.consume("<") {
                let lhs = Some(Box::new(node));
                let rhs = Some(Box::new(self.add()?));
                node = create_new_node(NodeKind::Comparison(ComparisonOpKind::Lt), lhs, rhs);
            } else if self.consume("<=") {
                let lhs = Some(Box::new(node));
                let rhs = Some(Box::new(self.add()?));
                node = create_new_node(NodeKind::Comparison(ComparisonOpKind::Le), lhs, rhs);
            } else if self.consume(">") {
                let lhs = Some(Box::new(node));
                let rhs = Some(Box::new(self.add()?));
                node = create_new_node(NodeKind::Comparison(ComparisonOpKind::Gt), lhs, rhs);
            } else if self.consume(">=") {
                let lhs = Some(Box::new(node));
                let rhs = Some(Box::new(self.add()?));
                node = create_new_node(NodeKind::Comparison(ComparisonOpKind::Ge), lhs, rhs);
            } else {
                return Ok(node);
            }
        }
    }

    fn add(&mut self) -> Result<Node, String> {
        let mut node = self.mul()?;

        loop {
            if self.consume("+") {
                let lhs = Some(Box::new(node));
                let rhs = Some(Box::new(self.mul()?));
                node = create_new_node(NodeKind::BinaryOp(BinaryOpKind::Add), lhs, rhs);
            } else if self.consume("-") {
                let lhs = Some(Box::new(node));
                let rhs = Some(Box::new(self.mul()?));
                node = create_new_node(NodeKind::BinaryOp(BinaryOpKind::Sub), lhs, rhs);
            } else {
                return Ok(node);
            }
        }
    }

    fn mul(&mut self) -> Result<Node, String> {
        let mut node = self.unary()?;

        loop {
            if self.consume("*") {
                let lhs = Some(Box::new(node));
                let rhs = Some(Box::new(self.unary()?));
                node = create_new_node(NodeKind::BinaryOp(BinaryOpKind::Mul), lhs, rhs);
            } else if self.consume("/") {
                let lhs = Some(Box::new(node));
                let rhs = Some(Box::new(self.unary()?));
                node = create_new_node(NodeKind::BinaryOp(BinaryOpKind::Div), lhs, rhs);
            } else {
                return Ok(node);
            }
        }
    }

    fn unary(&mut self) -> Result<Node, String> {
        if self.consume("+") {
            return self.primary();
        } else if self.consume("-") {
            // 0 - x として扱う
            let lhs = Some(Box::new(create_new_node(NodeKind::Num(0), None, None)));
            let rhs = Some(Box::new(self.primary()?));
            return Ok(create_new_node(
                NodeKind::BinaryOp(BinaryOpKind::Sub),
                lhs,
                rhs,
            ));
        } else if self.consume("&") {
            let lhs = Some(Box::new(self.unary()?));
            return Ok(create_new_node(
                NodeKind::UnaryOp(UnaryOpKind::Ref),
                lhs,
                None,
            ));
        } else if self.consume("*") {
            let lhs = Some(Box::new(self.unary()?));
            return Ok(create_new_node(
                NodeKind::UnaryOp(UnaryOpKind::Deref),
                lhs,
                None,
            ));
        } else if self.consume("sizeof") {
            let node = self.unary()?;

            if node.ty.kind == TypeKind::Int {
                return Ok(create_new_node(NodeKind::Num(4), None, None));
            } else {
                return Ok(create_new_node(NodeKind::Num(8), None, None));
            }
        } else {
            return self.primary();
        }
    }

    fn primary(&mut self) -> Result<Node, String> {
        if self.consume("(") {
            let node = self.expr()?;
            self.expect(")")?;
            Ok(node)
        } else if self.tokens[self.pos].kind == TokenKind::Ident {
            let name = self.tokens[self.pos].str.clone();
            let lvar = self.find_lvar(&name).cloned();
            let func = Function {
                name: name.clone(),
                // 一旦戻り値の型は適当にする
                // sizeofとか実装したらちゃんとやる
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            };
            self.pos += 1;

            if self.consume("(") {
                let mut args = Vec::new();
                if self.consume(")") {
                    return Ok(create_new_node(NodeKind::Fncall(func, args), None, None));
                } else {
                    args = self.arglist()?;
                    return Ok(create_new_node(NodeKind::Fncall(func, args), None, None));
                }
            } else {
                if lvar.is_none() {
                    return Err(format!("変数 '{}' が見つかりません", name));
                } else {
                    return Ok(create_new_node(NodeKind::LVar(lvar.unwrap()), None, None));
                }
            }
        } else {
            Ok(create_new_node(
                NodeKind::Num(self.expect_number()?),
                None,
                None,
            ))
        }
    }

    fn arglist(&mut self) -> Result<Vec<Node>, String> {
        let mut args = Vec::new();
        args.push(self.expr()?);

        loop {
            if self.consume(",") {
                args.push(self.expr()?);
            } else {
                break;
            }
        }

        self.expect(")")?;
        Ok(args)
    }

    fn ty(&mut self) -> Result<Type, String> {
        let mut ty = self.base_type()?;

        loop {
            if self.consume("*") {
                ty = Type {
                    kind: TypeKind::Ptr,
                    ptr_to: Some(Box::new(ty)),
                };
            } else {
                break;
            }
        }

        Ok(ty)
    }

    fn base_type(&mut self) -> Result<Type, String> {
        if self.consume("int") {
            Ok(Type {
                kind: TypeKind::Int,
                ptr_to: None,
            })
        } else {
            Err(format!(
                "intが期待されますが、{}でした",
                self.tokens[self.pos].str
            ))
        }
    }
}
