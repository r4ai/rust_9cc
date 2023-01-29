use crate::tokenize::{Token, TokenKind, Tokens};

#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Lt,     // <
    Le,     // <=
    Eq,     // ==
    Ne,     // !=
    Assign, // =
    LVar,   // local variable
    Num,    // integer
    Nil,    // empty node
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub kind: NodeKind,
    pub val: i64,    // kindがNumの場合のみ使う
    pub offset: i64, // kindがLVarの場合のみ使う
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

impl Node {
    pub fn default() -> Self {
        Self {
            kind: NodeKind::Nil,
            val: 0,
            offset: 0,
            lhs: None,
            rhs: None,
        }
    }

    pub fn new_num(val: i64) -> Self {
        Self {
            kind: NodeKind::Num,
            val,
            offset: 0,
            lhs: None,
            rhs: None,
        }
    }

    pub fn new_op(kind: NodeKind, lhs: Node, rhs: Node) -> Self {
        Self {
            kind,
            val: 0,
            offset: 0,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LVar {
    pub name: String,
    pub len: usize,
    pub offset: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LVars {
    pub vec: Vec<LVar>,
}

impl LVars {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn find(&self, name: &str) -> Option<&LVar> {
        self.vec.iter().find(|v| v.name == name)
    }
}

/// program = stmt*
pub fn program(tokens: &mut Tokens) -> Vec<Node> {
    let mut code = Vec::with_capacity(1);
    while tokens.len() > 0 {
        code.push(stmt(tokens));
    }
    code
}

/// stmt = expr ";"
pub fn stmt(tokens: &mut Tokens) -> Node {
    let node = expr(tokens);
    if !tokens.consume_op(";") {
        eprintln!("文末には';'が必要です。");
        std::process::exit(1);
    }
    node
}

/// expr = assign
pub fn expr(tokens: &mut Tokens) -> Node {
    assign(tokens)
}

/// assign = equality ("=" assign)?
fn assign(tokens: &mut Tokens) -> Node {
    let mut node = equality(tokens);
    if tokens.consume_op("=") {
        node = Node::new_op(NodeKind::Assign, node, assign(tokens));
    }
    node
}

/// equality = relational ("==" relational | "!=" relational)*
fn equality(tokens: &mut Tokens) -> Node {
    let mut node = relational(tokens);
    loop {
        if tokens.consume_op("==") {
            node = Node::new_op(NodeKind::Eq, node, relational(tokens));
        } else if tokens.consume_op("!=") {
            node = Node::new_op(NodeKind::Ne, node, relational(tokens));
        } else {
            return node;
        }
    }
}

/// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
fn relational(tokens: &mut Tokens) -> Node {
    let mut node = add(tokens);
    loop {
        if tokens.consume_op("<") {
            node = Node::new_op(NodeKind::Lt, node, add(tokens));
        } else if tokens.consume_op("<=") {
            node = Node::new_op(NodeKind::Le, node, add(tokens));
        } else if tokens.consume_op(">") {
            node = Node::new_op(NodeKind::Lt, add(tokens), node);
        } else if tokens.consume_op(">=") {
            node = Node::new_op(NodeKind::Le, add(tokens), node);
        } else {
            return node;
        }
    }
}

/// add = mul ("+" mul | "-" mul)*
fn add(tokens: &mut Tokens) -> Node {
    let mut node = mul(tokens);
    loop {
        if tokens.consume_op("+") {
            node = Node::new_op(NodeKind::Add, node, mul(tokens));
        } else if tokens.consume_op("-") {
            node = Node::new_op(NodeKind::Sub, node, mul(tokens));
        } else {
            return node;
        }
    }
}

/// mul = unary ("*" unary | "/" unary)*
fn mul(tokens: &mut Tokens) -> Node {
    let mut node = unary(tokens);
    loop {
        if tokens.consume_op("*") {
            node = Node::new_op(NodeKind::Mul, node, unary(tokens));
        } else if tokens.consume_op("/") {
            node = Node::new_op(NodeKind::Div, node, unary(tokens));
        } else {
            return node;
        }
    }
}

/// unary = ("+" | "-")? primary
fn unary(tokens: &mut Tokens) -> Node {
    if tokens.consume_op("+") {
        primary(tokens)
    } else if tokens.consume_op("-") {
        Node::new_op(NodeKind::Sub, Node::new_num(0), primary(tokens))
    } else {
        primary(tokens)
    }
}

/// primary = "(" expr ")" | ident | num
fn primary(tokens: &mut Tokens) -> Node {
    if tokens.consume_op("(") {
        let node = expr(tokens);
        if !tokens.consume_op(")") {
            panic!("')' is not found");
        };
        return node;
    } else if let Some(tk) = tokens.front() {
        match tk.kind {
            TokenKind::Ident => {
                if let Some(lvar) = tokens.lvars.find(&tk.str) {
                    let node = Node {
                        kind: NodeKind::LVar,
                        offset: lvar.offset,
                        ..Node::default()
                    };
                    tokens.pop_front();
                    return node;
                } else {
                    let lvar = LVar {
                        name: tk.str.clone(),
                        offset: tokens.lvars.vec.len() as i64 * 8 + 8,
                        len: tk.len,
                    };
                    let node = Node {
                        kind: NodeKind::LVar,
                        offset: lvar.offset,
                        ..Node::default()
                    };
                    tokens.lvars.vec.push(lvar);
                    tokens.pop_front();
                    return node;
                }
            }
            TokenKind::Num => {
                let node = Node::new_num(expect_number(&tokens.pop_front()));
                return node;
            }
            _ => panic!("数でも識別子でもないトークンです: {}", tk.str),
        }
    } else {
        panic!("解析エラー")
    }
}

fn expect_number(tk: &Option<Token>) -> i64 {
    match tk {
        Some(v) => match v.kind {
            TokenKind::Num => v.val,
            _ => panic!("数ではありません: {}", v.str),
        },
        None => panic!("解析エラー"),
    }
}

#[cfg(test)]
mod tests {
    use super::program;
    use crate::parse::{Node, NodeKind};
    use crate::tokenize::tokenize;

    #[test]
    fn check_ast_with_add() {
        let mut tokens = tokenize("1 + 2;".to_string()).unwrap();
        let actual_node = &program(&mut tokens)[0];
        let expected = Node {
            kind: NodeKind::Add,
            lhs: Some(Box::new(Node::new_num(1))),
            rhs: Some(Box::new(Node::new_num(2))),
            ..Node::default()
        };
        assert_eq!(actual_node, &expected);
    }

    #[test]
    fn check_ast_with_sub() {
        let mut tokens = tokenize("1 - 2;".to_string()).unwrap();
        let actual_node = &program(&mut tokens)[0];
        let expected = Node {
            kind: NodeKind::Sub,
            lhs: Some(Box::new(Node::new_num(1))),
            rhs: Some(Box::new(Node::new_num(2))),
            ..Node::default()
        };
        assert_eq!(actual_node, &expected);
    }

    #[test]
    fn check_ast_with_add_and_sub() {
        let mut tokens = tokenize("1 + 2 - 3;".to_string()).unwrap();
        let actual_node = &program(&mut tokens)[0];
        let expected = &Node {
            kind: NodeKind::Sub,
            lhs: Some(Box::new(Node {
                kind: NodeKind::Add,
                lhs: Some(Box::new(Node::new_num(1))),
                rhs: Some(Box::new(Node::new_num(2))),
                ..Node::default()
            })),
            rhs: Some(Box::new(Node::new_num(3))),
            ..Node::default()
        };
        assert_eq!(actual_node, expected);
    }

    #[test]
    fn check_ast_with_multipy() {
        let mut tokens = tokenize("1 + 2 * 3;".to_string()).unwrap();
        let actual_node = &program(&mut tokens)[0];
        let expected = &Node {
            kind: NodeKind::Add,
            lhs: Some(Box::new(Node::new_num(1))),
            rhs: Some(Box::new(Node {
                kind: NodeKind::Mul,
                lhs: Some(Box::new(Node::new_num(2))),
                rhs: Some(Box::new(Node::new_num(3))),
                ..Node::default()
            })),
            ..Node::default()
        };
        assert_eq!(
            actual_node, expected,
            "`1 + 2 * 3` の得られたAST:\n{actual_node:?}"
        );
    }

    #[test]
    fn check_ast_with_division() {
        let mut tokens = tokenize("4 / 2 - 2;".to_string()).unwrap();
        let node = &program(&mut tokens)[0];
        assert_eq!(
            node,
            &Node {
                kind: NodeKind::Sub,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Div,
                    lhs: Some(Box::new(Node::new_num(4))),
                    rhs: Some(Box::new(Node::new_num(2))),
                    ..Node::default()
                })),
                rhs: Some(Box::new(Node::new_num(2))),
                ..Node::default()
            },
            "`4 / 2 - 2` の得られたAST:\n{node:?}"
        );
    }

    #[test]
    fn check_ast_with_parenthesis() {
        let mut tokens = tokenize("1 * 2+(3+4);".to_string()).unwrap();
        let node = &program(&mut tokens)[0];
        assert_eq!(
            node,
            &Node {
                kind: NodeKind::Add,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Mul,
                    lhs: Some(Box::new(Node::new_num(1))),
                    rhs: Some(Box::new(Node::new_num(2))),
                    ..Node::default()
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Add,
                    lhs: Some(Box::new(Node::new_num(3))),
                    rhs: Some(Box::new(Node::new_num(4))),
                    ..Node::default()
                })),
                ..Node::default()
            },
            "`1 * 2+(3+4)` の得られたAST:\n{node:?}"
        );
    }

    #[test]
    fn check_ast_with_unary_operator() {
        let mut tokens = tokenize("-1 + 2;".to_string()).unwrap();
        let node = &program(&mut tokens)[0];
        assert_eq!(
            node,
            &Node {
                kind: NodeKind::Add,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Sub,
                    lhs: Some(Box::new(Node::new_num(0))),
                    rhs: Some(Box::new(Node::new_num(1))),
                    ..Node::default()
                })),
                rhs: Some(Box::new(Node::new_num(2))),
                ..Node::default()
            },
            "`-1 + 2` の得られたAST:\n{node:?}"
        );
    }

    #[test]
    fn check_ast_with_unary_operator_complecated() {
        let mut tokens = tokenize("-3*+5 + 20;".to_string()).unwrap();
        let node = &program(&mut tokens)[0];
        assert_eq!(
            node,
            &Node {
                kind: NodeKind::Add,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Mul,
                    lhs: Some(Box::new(Node {
                        kind: NodeKind::Sub,
                        lhs: Some(Box::new(Node::new_num(0))),
                        rhs: Some(Box::new(Node::new_num(3))),
                        ..Node::default()
                    })),
                    rhs: Some(Box::new(Node::new_num(5))),
                    ..Node::default()
                })),
                rhs: Some(Box::new(Node::new_num(20))),
                ..Node::default()
            },
            "`-3*+5 + 20` の得られたAST:\n{node:?}"
        );
    }

    #[test]
    fn check_ast_with_lt_operator() {
        let mut tokens = tokenize("1 < 2;".to_string()).unwrap();
        let node = &program(&mut tokens)[0];
        assert_eq!(
            node,
            &Node {
                kind: NodeKind::Lt,
                lhs: Some(Box::new(Node::new_num(1))),
                rhs: Some(Box::new(Node::new_num(2))),
                ..Node::default()
            },
            "`1 < 2` の得られたAST:\n{node:?}"
        );
    }

    #[test]
    fn check_ast_with_le_operator() {
        let mut tokens = tokenize("1 <= 2;".to_string()).unwrap();
        let node = &program(&mut tokens)[0];
        assert_eq!(
            node,
            &Node {
                kind: NodeKind::Le,
                lhs: Some(Box::new(Node::new_num(1))),
                rhs: Some(Box::new(Node::new_num(2))),
                ..Node::default()
            },
            "`1 <= 2` の得られたAST:\n{node:?}"
        );
    }

    #[test]
    fn check_ast_with_eq_operator() {
        let mut tokens = tokenize("1 == 2;".to_string()).unwrap();
        let node = &program(&mut tokens)[0];
        assert_eq!(
            node,
            &Node {
                kind: NodeKind::Eq,
                lhs: Some(Box::new(Node::new_num(1))),
                rhs: Some(Box::new(Node::new_num(2))),
                ..Node::default()
            },
            "`1 == 2` の得られたAST:\n{node:?}"
        );
    }

    #[test]
    fn check_ast_with_ne_operator() {
        let mut tokens = tokenize("1 != 2;".to_string()).unwrap();
        let node = &program(&mut tokens)[0];
        assert_eq!(
            node,
            &Node {
                kind: NodeKind::Ne,
                lhs: Some(Box::new(Node::new_num(1))),
                rhs: Some(Box::new(Node::new_num(2))),
                ..Node::default()
            },
            "`1 != 2` の得られたAST:\n{node:?}"
        );
    }

    #[test]
    fn check_ast_with_local_variables() {
        let mut tokens = tokenize("a = 2 * 3; b = 3 + -2; a * b;".to_string()).unwrap();
        let nodes = program(&mut tokens);
        assert_eq!(nodes.len(), 3);
        assert_eq!(
            &nodes[0],
            &Node {
                kind: NodeKind::Assign,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::LVar,
                    offset: ('a' as i64 - 'a' as i64 + 1) * 8,
                    ..Node::default()
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Mul,
                    lhs: Some(Box::new(Node::new_num(2))),
                    rhs: Some(Box::new(Node::new_num(3))),
                    ..Node::default()
                })),
                ..Node::default()
            },
            "`a = 2 * 3;` の得られたAST:\n{:?}",
            &nodes[0]
        );
        assert_eq!(
            &nodes[1],
            &Node {
                kind: NodeKind::Assign,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::LVar,
                    offset: ('b' as i64 - 'a' as i64 + 1) * 8,
                    ..Node::default()
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Add,
                    lhs: Some(Box::new(Node::new_num(3))),
                    rhs: Some(Box::new(Node {
                        kind: NodeKind::Sub,
                        lhs: Some(Box::new(Node::new_num(0))),
                        rhs: Some(Box::new(Node::new_num(2))),
                        ..Node::default()
                    })),
                    ..Node::default()
                })),
                ..Node::default()
            },
            "`b = 3 + -2;` の得られたAST:\n{:?}",
            &nodes[1]
        );
        assert_eq!(
            &nodes[2],
            &Node {
                kind: NodeKind::Mul,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::LVar,
                    offset: ('a' as i64 - 'a' as i64 + 1) * 8,
                    ..Node::default()
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::LVar,
                    offset: ('b' as i64 - 'a' as i64 + 1) * 8,
                    ..Node::default()
                })),
                ..Node::default()
            },
            "`a * b;` の得られたAST:\n{:?}",
            &nodes[2]
        );
    }

    #[test]
    fn check_ast_with_multi_lines() {
        let mut tokens = tokenize("1 + 2; 3 + -4 * 3;".to_string()).unwrap();
        let nodes = program(&mut tokens);
        assert_eq!(nodes.len(), 2);
        let node_1 = &nodes[0];
        let node_2 = &nodes[1];
        assert_eq!(
            node_1,
            &Node {
                kind: NodeKind::Add,
                lhs: Some(Box::new(Node::new_num(1))),
                rhs: Some(Box::new(Node::new_num(2))),
                ..Node::default()
            },
            "`1 + 2;` の得られたAST:\n{node_1:?}"
        );
        assert_eq!(
            node_2,
            &Node {
                kind: NodeKind::Add,
                lhs: Some(Box::new(Node::new_num(3))),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Mul,
                    lhs: Some(Box::new(Node {
                        kind: NodeKind::Sub,
                        lhs: Some(Box::new(Node::new_num(0))),
                        rhs: Some(Box::new(Node::new_num(4))),
                        ..Node::default()
                    })),
                    rhs: Some(Box::new(Node::new_num(3))),
                    ..Node::default()
                })),
                ..Node::default()
            },
            "`3 + -4 * 3;` の得られたAST:\n{node_2:?}"
        );
    }
}
