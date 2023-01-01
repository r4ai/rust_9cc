mod result;
mod tokenize;

use std::collections::VecDeque;

use result::TokenizeResult;
use tokenize::{tokenize, Token, TokenKind, Tokens};

#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num,
    Nil,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub kind: NodeKind,
    pub val: i64,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

impl Node {
    pub fn init() -> Self {
        Self {
            kind: NodeKind::Nil,
            val: 0,
            lhs: None,
            rhs: None,
        }
    }

    pub fn new_op(op: char) -> TokenizeResult<Self> {
        match op {
            '+' => Ok(Self {
                kind: NodeKind::Add,
                val: 0,
                lhs: None,
                rhs: None,
            }),
            '-' => Ok(Self {
                kind: NodeKind::Sub,
                val: 0,
                lhs: None,
                rhs: None,
            }),
            '*' => Ok(Self {
                kind: NodeKind::Mul,
                val: 0,
                lhs: None,
                rhs: None,
            }),
            '/' => Ok(Self {
                kind: NodeKind::Div,
                val: 0,
                lhs: None,
                rhs: None,
            }),
            _ => Err(result::TokenizeError::InvalidOperator(op)),
        }
    }
}

fn base(tokens: &mut VecDeque<Token>) -> Node {
    todo!()
}

fn expr(tokens: &mut Tokens) -> Node {
    let mut node_center = Node::init();
    let mut node_right = Node::init();
    let node_left = mul(tokens);
    while tokens.tokens.len() > 0 {
        if tokens.consume_op('+') {
            node_center = Node::new_op('+').unwrap();
            node_right = mul(tokens);
        } else if tokens.consume_op('-') {
            node_center = Node::new_op('-').unwrap();
            node_right = mul(tokens);
        } else {
            break;
        }
    }

    if node_center.kind == NodeKind::Nil {
        node_left
    } else {
        node_center.lhs = Some(Box::new(node_left));
        node_center.rhs = match node_right.kind {
            NodeKind::Nil => None,
            _ => Some(Box::new(node_right)),
        };
        node_center
    }
}

fn mul(tokens: &mut Tokens) -> Node {
    let mut node_center = Node::init();
    let mut node_right = Node::init();
    let node_left = primary(tokens);
    while tokens.tokens.len() > 0 {
        if tokens.consume_op('*') {
            node_center = Node::new_op('*').unwrap();
            node_right = primary(tokens);
        } else if tokens.consume_op('/') {
            node_center = Node::new_op('/').unwrap();
            node_right = primary(tokens);
        } else {
            break;
        }
    }

    if node_center.kind == NodeKind::Nil {
        node_left
    } else {
        node_center.lhs = Some(Box::new(node_left));
        node_center.rhs = match node_right.kind {
            NodeKind::Nil => None,
            _ => Some(Box::new(node_right)),
        };
        node_center
    }
}

fn primary(tokens: &mut Tokens) -> Node {
    if tokens.consume_op('(') {
        let node = expr(tokens);
        if !tokens.consume_op(')') {
            panic!("')' is not found");
        };
        return node;
    } else {
        let node = Node {
            kind: NodeKind::Num,
            val: expect_number(&tokens.pop_front()),
            lhs: None,
            rhs: None,
        };
        return node;
    }
}

fn expect_number(tk: &Option<Token>) -> i64 {
    match tk {
        Some(v) => match v.kind {
            TokenKind::Num => v.val,
            _ => panic!("数ではありません: {}", v.char),
        },
        None => panic!("解析エラー"),
    }
}

pub fn cli(args: Vec<String>) -> String {
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let mut tokens = tokenize(args[1].to_string()).unwrap();
    let mut result = String::new();

    result.push_str(".intel_syntax noprefix\n");
    result.push_str(".globl main\n");
    result.push_str("main:\n");

    let node = expr(&mut tokens);
    dbg!(node);

    result.push_str("  ret\n");
    return result;
}

#[cfg(test)]
mod tests {
    use super::{expr, tokenize, Node, NodeKind};

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
}
