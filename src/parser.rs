use crate::error;
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
    Arr,
    Char,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub kind: TypeKind,
    pub ptr_to: Option<Box<Type>>,
    pub arr_size: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LVar {
    pub name: String,
    pub offset: usize,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GVar {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub stack_size: usize,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    BinaryOp(BinaryOpKind),       // Binaty operations: +, -, *, /
    UnaryOp(UnaryOpKind),         // Unary operations: &, *
    Comparison(ComparisonOpKind), // Comparison operations: ==, !=, <, <=, >, >=
    Num(i32),                     // Numeric literals
    LVar(LVar),                   // Local variable
    GVar(GVar),                   // Global variable
    GVarDef(GVar),                // Global variable definition
    Assign,                       // Assignment
    Return,                       // Return statement
    Block(Vec<Node>),             // Block of statements
    Fncall(Function, Vec<Node>),  // Function call with arguments
    Fndef(Function, Vec<Node>),   // Function definition (name, parameters)
    For,                          // For
    While,                        // While
    If,                           // If
    Else,                         // Else
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
    pub globals: HashMap<String, GVar>,
    pub functions: Vec<Function>,
    stack_size: usize,
}

pub fn get_type_size(ty: &Type) -> usize {
    match &ty.kind {
        TypeKind::Char => 1,
        TypeKind::Int => 4,
        TypeKind::Ptr => 8,
        TypeKind::Arr => ty.arr_size * get_type_size(ty.ptr_to.as_ref().unwrap()),
    }
}

// 左右の子に応じてノードの型を決定する
fn create_new_node(kind: NodeKind, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Node {
    let ty = match &kind {
        NodeKind::BinaryOp(op) => {
            match op {
                BinaryOpKind::Add | BinaryOpKind::Sub => {
                    if let (Some(l), Some(r)) = (&lhs, &rhs) {
                        match (&l.ty.kind, &r.ty.kind) {
                            (TypeKind::Ptr, TypeKind::Int) | (TypeKind::Arr, TypeKind::Int) => {
                                // ポインタ/配列 + 整数 の場合
                                match &l.ty.ptr_to {
                                    Some(ptr_to) if ptr_to.kind == TypeKind::Arr => {
                                        // 多次元配列の場合、1次元分の型を進める
                                        (**ptr_to).clone()
                                    }
                                    _ => l.ty.clone(),
                                }
                            }
                            (TypeKind::Int, TypeKind::Ptr) | (TypeKind::Int, TypeKind::Arr) => {
                                // 整数 + ポインタ/配列 の場合
                                match &r.ty.ptr_to {
                                    Some(ptr_to) if ptr_to.kind == TypeKind::Arr => {
                                        // 多次元配列の場合、1次元分の型を進める
                                        (**ptr_to).clone()
                                    }
                                    _ => r.ty.clone(),
                                }
                            }
                            _ => Type {
                                kind: TypeKind::Int,
                                ptr_to: None,
                                arr_size: 1,
                            },
                        }
                    } else {
                        Type {
                            kind: TypeKind::Int,
                            ptr_to: None,
                            arr_size: 1,
                        }
                    }
                }
                BinaryOpKind::Mul | BinaryOpKind::Div => Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                    arr_size: 1,
                },
            }
        }
        NodeKind::UnaryOp(op) => match op {
            UnaryOpKind::Ref => {
                if let Some(l) = &lhs {
                    Type {
                        kind: TypeKind::Ptr,
                        ptr_to: Some(Box::new(l.ty.clone())),
                        arr_size: 1,
                    }
                } else {
                    Type {
                        kind: TypeKind::Ptr,
                        ptr_to: None,
                        arr_size: 1,
                    }
                }
            }
            UnaryOpKind::Deref => {
                if let Some(l) = &lhs {
                    match &l.ty.kind {
                        TypeKind::Ptr | TypeKind::Arr => l.ty.ptr_to.as_ref().map_or(
                            Type {
                                kind: TypeKind::Int,
                                ptr_to: None,
                                arr_size: 1,
                            },
                            |ptr_to| (**ptr_to).clone(),
                        ),
                        _ => Type {
                            kind: TypeKind::Int,
                            ptr_to: None,
                            arr_size: 1,
                        },
                    }
                } else {
                    Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                        arr_size: 1,
                    }
                }
            }
        },
        _ => match &kind {
            NodeKind::Comparison(_) => Type {
                kind: TypeKind::Int,
                ptr_to: None,
                arr_size: 1,
            },
            NodeKind::Num(_) => Type {
                kind: TypeKind::Int,
                ptr_to: None,
                arr_size: 1,
            },
            NodeKind::LVar(lvar) => lvar.ty.clone(),
            NodeKind::GVar(gvar) | NodeKind::GVarDef(gvar) => gvar.ty.clone(),
            NodeKind::Assign => {
                if let Some(l) = &lhs {
                    l.ty.clone()
                } else {
                    Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                        arr_size: 1,
                    }
                }
            }
            NodeKind::Return => Type {
                kind: TypeKind::Int,
                ptr_to: None,
                arr_size: 1,
            },
            NodeKind::Block(_) => Type {
                kind: TypeKind::Int,
                ptr_to: None,
                arr_size: 1,
            },
            NodeKind::Fncall(func, _) => func.ty.clone(),
            NodeKind::Fndef(func, _) => func.ty.clone(),
            NodeKind::For | NodeKind::While | NodeKind::If | NodeKind::Else => Type {
                kind: TypeKind::Int,
                ptr_to: None,
                arr_size: 1,
            },
            _ => unreachable!(),
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
            globals: HashMap::new(),
            functions: Vec::new(),
            stack_size: 8,
        }
    }

    fn create_lvar(&mut self, name: &str, ty: Type) -> LVar {
        let offset = self.stack_size;
        // self.stack_size += get_type_size(&ty);
        if ty.kind == TypeKind::Char || ty.kind == TypeKind::Int || ty.kind == TypeKind::Ptr {
            self.stack_size += 8;
        } else if ty.kind == TypeKind::Arr {
            let mut ty_iter = ty.clone();
            let mut elem_cnt = 1;
            while ty_iter.ptr_to.is_some() {
                elem_cnt *= ty_iter.arr_size;
                ty_iter = *ty_iter.ptr_to.unwrap();
            }
            self.stack_size += 8 * elem_cnt;
        }
        let lvar = LVar {
            name: name.to_string(),
            offset: offset,
            ty: ty.clone(),
        };
        self.locals[self.fn_idx].insert(name.to_string(), lvar.clone());
        lvar
    }

    fn find_lvar(&self, name: &str) -> Option<&LVar> {
        self.locals[self.fn_idx].get(name)
    }

    fn create_gvar(&mut self, name: &str, ty: Type) -> GVar {
        let gvar = GVar {
            name: name.to_string(),
            ty: ty.clone(),
        };
        self.globals.insert(name.to_string(), gvar.clone());
        gvar
    }

    fn find_gvar(&self, name: &str) -> Option<&GVar> {
        self.globals.get(name)
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
        let mut ty;
        while !self.at_eof() {
            ty = self.ty()?;
            if self.tokens[self.pos].kind != TokenKind::Ident {
                error::error("変数名がありません");
            }
            let name = self.tokens[self.pos].str.clone();
            self.pos += 1;

            if self.tokens[self.pos].str == "(" {
                self.locals.push(HashMap::new());
                nodes.push(self.function(name, ty)?);
                self.fn_idx += 1;
                self.stack_size = 8;
            } else {
                nodes.push(self.global_decl(name, ty)?);
            }
        }
        return Ok(nodes);
    }

    fn function(&mut self, name: String, ty: Type) -> Result<Node, String> {
        self.expect("(")?;

        let mut params = Vec::new();
        if !self.consume(")") {
            params = self.paramlist()?;
            self.expect(")")?;
        }

        if self.functions.iter().any(|x| x.name == name) {
            return Err(format!("関数 '{}' はすでに定義されています", name));
        }

        let body = self.stmt()?;

        let rhs = Some(Box::new(body));
        let func = Function {
            name: name.clone(),
            stack_size: self.stack_size,
            ty: ty.clone(),
        };
        self.functions.push(func.clone());
        Ok(create_new_node(
            NodeKind::Fndef(func.clone(), params.clone()),
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

    fn global_decl(&mut self, name: String, mut ty: Type) -> Result<Node, String> {
        let node;

        let mut nums = Vec::new();
        while self.consume("[") {
            nums.push(self.expect_number()?);
            self.expect("]")?;
        }

        for num in nums.iter().rev() {
            ty.ptr_to = Some(Box::new(ty.clone()));
            ty.kind = TypeKind::Arr;
            ty.arr_size = *num as usize;
        }

        let gvar = self.create_gvar(&name, ty.clone());
        self.globals.insert(name.clone(), gvar.clone());

        node = create_new_node(NodeKind::GVarDef(gvar.clone()), None, None);

        self.expect(";")?;
        Ok(node)
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
        } else if self.tokens[self.pos].str == "int" || self.tokens[self.pos].str == "char" {
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
        let mut ty = self.ty()?;
        let name = self.tokens[self.pos].str.clone();
        self.pos += 1;

        let mut nums = Vec::new();
        while self.consume("[") {
            nums.push(self.expect_number()?);
            self.expect("]")?;
        }

        for num in nums.iter().rev() {
            ty.ptr_to = Some(Box::new(ty.clone()));
            ty.kind = TypeKind::Arr;
            ty.arr_size = *num as usize;
        }

        let lvar = self.create_lvar(&name, ty.clone());

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
            let gvar = self.find_gvar(&name).cloned();
            let mut func = self.functions.iter().find(|&x| x.name == name).cloned();
            if func.is_none() {
                func = Some(Function {
                    name: name.clone(),
                    stack_size: 0,
                    ty: Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                        arr_size: 1,
                    },
                });
            }
            self.pos += 1;

            if self.consume("(") {
                let mut args = Vec::new();
                if self.consume(")") {
                    return Ok(create_new_node(
                        NodeKind::Fncall(func.unwrap(), args),
                        None,
                        None,
                    ));
                } else {
                    args = self.arglist()?;
                    return Ok(create_new_node(
                        NodeKind::Fncall(func.unwrap(), args),
                        None,
                        None,
                    ));
                }
            } else if self.consume("[") {
                if lvar.is_none() && gvar.is_none() {
                    return Err(format!("変数 '{}' が見つかりません", name));
                }

                let mut indices = Vec::new();
                indices.push(self.expr()?);
                self.expect("]")?;

                while self.consume("[") {
                    indices.push(self.expr()?);
                    self.expect("]")?;
                }

                let mut node;
                if lvar.is_some() {
                    node = create_new_node(NodeKind::LVar(lvar.unwrap()), None, None);
                } else {
                    node = create_new_node(NodeKind::GVar(gvar.unwrap()), None, None);
                }

                // a[3][4] -> *(*(a+3)+4)
                for index in indices.iter() {
                    node = create_new_node(
                        NodeKind::BinaryOp(BinaryOpKind::Add),
                        Some(Box::new(node)),
                        Some(Box::new(index.clone())),
                    );
                }
                Ok(create_new_node(
                    NodeKind::UnaryOp(UnaryOpKind::Deref),
                    Some(Box::new(node)),
                    None,
                ))
            } else {
                if lvar.is_none() && gvar.is_none() {
                    return Err(format!("変数 '{}' が見つかりません", name));
                } else {
                    if lvar.is_some() {
                        return Ok(create_new_node(NodeKind::LVar(lvar.unwrap()), None, None));
                    } else {
                        return Ok(create_new_node(NodeKind::GVar(gvar.unwrap()), None, None));
                    }
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
                    arr_size: 1,
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
                arr_size: 1,
            })
        } else if self.consume("char") {
            Ok(Type {
                kind: TypeKind::Char,
                ptr_to: None,
                arr_size: 1,
            })
        } else {
            Err(format!(
                "型名cが期待されますが、{}でした",
                self.tokens[self.pos].str
            ))
        }
    }
}
