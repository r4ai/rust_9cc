use crate::tokenize::{Token, TokenKind, Tokens};

#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Le,
    Eq,
    Ne,
    Num,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub kind: NodeKind,
    pub val: i64,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

impl Node {
    pub fn new_num(val: i64) -> Self {
        Self {
            kind: NodeKind::Num,
            val,
            lhs: None,
            rhs: None,
        }
    }

    pub fn new(kind: NodeKind, lhs: Node, rhs: Node) -> Self {
        Self {
            kind,
            val: 0,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
        }
    }
}

pub fn expr(tokens: &mut Tokens) -> Node {
    let mut node = equality(tokens);
    loop {
        if tokens.consume_op("+") {
            node = Node::new(NodeKind::Add, node, equality(tokens));
        } else if tokens.consume_op("-") {
            node = Node::new(NodeKind::Sub, node, equality(tokens));
        } else {
            return node;
        }
    }
}

fn equality(tokens: &mut Tokens) -> Node {
    let mut node = relational(tokens);
    loop {
        if tokens.consume_op("==") {
            node = Node::new(NodeKind::Eq, node, relational(tokens));
        } else if tokens.consume_op("!=") {
            node = Node::new(NodeKind::Ne, node, relational(tokens));
        } else {
            return node;
        }
    }
}

fn relational(tokens: &mut Tokens) -> Node {
    let mut node = add(tokens);
    loop {
        if tokens.consume_op("<") {
            node = Node::new(NodeKind::Lt, node, add(tokens));
        } else if tokens.consume_op("<=") {
            node = Node::new(NodeKind::Le, node, add(tokens));
        } else if tokens.consume_op(">") {
            node = Node::new(NodeKind::Lt, add(tokens), node);
        } else if tokens.consume_op(">=") {
            node = Node::new(NodeKind::Le, add(tokens), node);
        } else {
            return node;
        }
    }
}

fn add(tokens: &mut Tokens) -> Node {
    let mut node = mul(tokens);
    loop {
        if tokens.consume_op("+") {
            node = Node::new(NodeKind::Add, node, mul(tokens));
        } else if tokens.consume_op("-") {
            node = Node::new(NodeKind::Sub, node, mul(tokens));
        } else {
            return node;
        }
    }
}

fn mul(tokens: &mut Tokens) -> Node {
    let mut node = unary(tokens);
    loop {
        if tokens.consume_op("*") {
            node = Node::new(NodeKind::Mul, node, unary(tokens));
        } else if tokens.consume_op("/") {
            node = Node::new(NodeKind::Div, node, unary(tokens));
        } else {
            return node;
        }
    }
}

fn unary(tokens: &mut Tokens) -> Node {
    if tokens.consume_op("+") {
        primary(tokens)
    } else if tokens.consume_op("-") {
        Node::new(NodeKind::Sub, Node::new_num(0), primary(tokens))
    } else {
        primary(tokens)
    }
}

fn primary(tokens: &mut Tokens) -> Node {
    if tokens.consume_op("(") {
        let node = expr(tokens);
        if !tokens.consume_op(")") {
            panic!("')' is not found");
        };
        return node;
    } else {
        let node = Node::new_num(expect_number(&tokens.pop_front()));
        return node;
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
    use super::expr;
    use crate::parse::{Node, NodeKind};
    use crate::tokenize::tokenize;

    #[test]
    fn check_ast_with_add() {
        let mut tokens = tokenize("1 + 2".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Add,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 1,
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 2,
                    lhs: None,
                    rhs: None,
                })),
            }
        );
    }

    #[test]
    fn check_ast_with_sub() {
        let mut tokens = tokenize("1 - 2".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Sub,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 1,
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 2,
                    lhs: None,
                    rhs: None,
                })),
            }
        );
    }

    #[test]
    fn check_ast_with_add_and_sub() {
        let mut tokens = tokenize("1 + 2 - 3".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Sub,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Add,
                    val: 0,
                    lhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 1,
                        lhs: None,
                        rhs: None,
                    })),
                    rhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 2,
                        lhs: None,
                        rhs: None,
                    })),
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 3,
                    lhs: None,
                    rhs: None,
                })),
            }
        );
    }

    #[test]
    fn check_ast_with_multipy() {
        let mut tokens = tokenize("1 + 2 * 3".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Add,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 1,
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Mul,
                    val: 0,
                    lhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 2,
                        lhs: None,
                        rhs: None,
                    })),
                    rhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 3,
                        lhs: None,
                        rhs: None,
                    })),
                })),
            },
            "`1 + 2 * 3` の得られたAST:\n{:?}",
            node
        );
    }

    #[test]
    fn check_ast_with_division() {
        let mut tokens = tokenize("4 / 2 - 2".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Sub,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Div,
                    val: 0,
                    lhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 4,
                        lhs: None,
                        rhs: None,
                    })),
                    rhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 2,
                        lhs: None,
                        rhs: None,
                    })),
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 2,
                    lhs: None,
                    rhs: None,
                })),
            },
            "`4 / 2 - 2` の得られたAST:\n{:?}",
            node
        );
    }

    #[test]
    fn check_ast_with_parenthesis() {
        let mut tokens = tokenize("1 * 2+(3+4)".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Add,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Mul,
                    val: 0,
                    lhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 1,
                        lhs: None,
                        rhs: None,
                    })),
                    rhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 2,
                        lhs: None,
                        rhs: None,
                    })),
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Add,
                    val: 0,
                    lhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 3,
                        lhs: None,
                        rhs: None,
                    })),
                    rhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 4,
                        lhs: None,
                        rhs: None,
                    })),
                })),
            },
            "`1 * 2+(3+4)` の得られたAST:\n{:?}",
            node
        );
    }

    #[test]
    fn check_ast_with_unary_operator() {
        let mut tokens = tokenize("-1 + 2".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Add,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Sub,
                    val: 0,
                    lhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 0,
                        lhs: None,
                        rhs: None,
                    })),
                    rhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 1,
                        lhs: None,
                        rhs: None,
                    })),
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 2,
                    lhs: None,
                    rhs: None,
                })),
            },
            "`-1 + 2` の得られたAST:\n{:?}",
            node
        );
    }

    #[test]
    fn check_ast_with_unary_operator_complecated() {
        let mut tokens = tokenize("-3*+5 + 20".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Add,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Mul,
                    val: 0,
                    lhs: Some(Box::new(Node {
                        kind: NodeKind::Sub,
                        val: 0,
                        lhs: Some(Box::new(Node {
                            kind: NodeKind::Num,
                            val: 0,
                            lhs: None,
                            rhs: None,
                        })),
                        rhs: Some(Box::new(Node {
                            kind: NodeKind::Num,
                            val: 3,
                            lhs: None,
                            rhs: None,
                        })),
                    })),
                    rhs: Some(Box::new(Node {
                        kind: NodeKind::Num,
                        val: 5,
                        lhs: None,
                        rhs: None
                    }))
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 20,
                    lhs: None,
                    rhs: None,
                })),
            },
            "`-3*+5 + 20` の得られたAST:\n{:?}",
            node
        );
    }

    #[test]
    fn check_ast_with_lt_operator() {
        let mut tokens = tokenize("1 < 2".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Lt,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 1,
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 2,
                    lhs: None,
                    rhs: None,
                })),
            },
            "`1 < 2` の得られたAST:\n{:?}",
            node
        );
    }

    #[test]
    fn check_ast_with_le_operator() {
        let mut tokens = tokenize("1 <= 2".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Le,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 1,
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 2,
                    lhs: None,
                    rhs: None,
                })),
            },
            "`1 <= 2` の得られたAST:\n{:?}",
            node
        );
    }

    #[test]
    fn check_ast_with_eq_operator() {
        let mut tokens = tokenize("1 == 2".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Eq,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 1,
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 2,
                    lhs: None,
                    rhs: None,
                })),
            },
            "`1 == 2` の得られたAST:\n{:?}",
            node
        );
    }

    #[test]
    fn check_ast_with_ne_operator() {
        let mut tokens = tokenize("1 != 2".to_string()).unwrap();
        let node = expr(&mut tokens);
        assert_eq!(
            node,
            Node {
                kind: NodeKind::Ne,
                val: 0,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 1,
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(Node {
                    kind: NodeKind::Num,
                    val: 2,
                    lhs: None,
                    rhs: None,
                })),
            },
            "`1 != 2` の得られたAST:\n{:?}",
            node
        );
    }
}
