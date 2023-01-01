mod result;
mod tokenize;

use std::collections::VecDeque;

use tokenize::{tokenize, Token, TokenKind};

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
}

fn base(tokens: &mut VecDeque<Token>) -> Node {
    todo!()
}

fn expr(tokens: &mut VecDeque<Token>) -> Node {
    let mut first_tk = VecDeque::from([tokens.pop_front().unwrap()]);
    let mut node_center = Node::init();
    let mut node_right = Node::init();
    let node_left = mul(&mut first_tk);
    while tokens.len() > 0 {
        match tokens.pop_front() {
            Some(tk) => {
                if tk.kind == TokenKind::Reserved {
                    match tk.char {
                        '+' => {
                            node_center = Node {
                                kind: NodeKind::Add,
                                val: 0,
                                lhs: None,
                                rhs: None,
                            };
                            node_right = mul(tokens);
                        }
                        '-' => {
                            node_center = Node {
                                kind: NodeKind::Sub,
                                val: 0,
                                lhs: None,
                                rhs: None,
                            };
                            node_right = mul(tokens);
                        }
                        _ => {
                            panic!("定義されていない演算子です: {}", &tk.char);
                        }
                    }
                } else {
                    panic!("演算子が来るはずです: {}", &tk.char);
                }
            }
            None => {
                panic!("存在しないトークンへのアクセスが発生しました");
            }
        };
    }
    if node_center.kind == NodeKind::Nil {
        return node_left;
    }
    node_center.lhs = Some(Box::new(node_left));
    node_center.rhs = match node_right.kind {
        NodeKind::Nil => None,
        _ => Some(Box::new(node_right)),
    };
    node_center
}

fn mul(tokens: &mut VecDeque<Token>) -> Node {
    let mut first_tk = VecDeque::from([tokens.pop_front().unwrap()]);
    let mut node_center = Node::init();
    let mut node_right = Node::init();
    let node_left = primary(&mut first_tk);
    while tokens.len() > 0 {
        match tokens.pop_front() {
            Some(tk) => {
                if tk.kind == TokenKind::Reserved {
                    match tk.char {
                        '*' => {
                            node_center = Node {
                                kind: NodeKind::Mul,
                                val: 0,
                                lhs: None,
                                rhs: None,
                            };
                            node_right = primary(tokens);
                        }
                        '/' => {
                            node_center = Node {
                                kind: NodeKind::Div,
                                val: 0,
                                lhs: None,
                                rhs: None,
                            };
                            node_right = primary(tokens);
                        }
                        _ => {
                            panic!("定義されていない演算子です: {}", &tk.char);
                        }
                    }
                } else {
                    panic!("演算子が来るはずです: {}", &tk.char);
                }
            }
            None => {
                panic!("存在しないトークンへのアクセスが発生しました");
            }
        };
    }
    if node_center.kind == NodeKind::Nil {
        return node_left;
    }
    node_center.lhs = Some(Box::new(node_left));
    node_center.rhs = match node_right.kind {
        NodeKind::Nil => None,
        _ => Some(Box::new(node_right)),
    };
    node_center
}

fn primary(tokens: &mut VecDeque<Token>) -> Node {
    let first_tk = tokens.front().unwrap();
    if first_tk.kind == TokenKind::Reserved && first_tk.char == '(' {
        consume(tokens, '(');
        let node = expr(tokens);
        consume(tokens, ')');
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

fn consume(tokens: &mut VecDeque<Token>, expect_char: char) -> Option<Token> {
    let first_tk = tokens.front().unwrap();
    if first_tk.kind == TokenKind::Reserved && first_tk.char == expect_char {
        return tokens.pop_front();
    } else {
        return None;
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

    let mut tokens = tokenize(args[1].to_string()).unwrap().tokens;
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
        let node = expr(&mut tokens.tokens);
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
        let node = expr(&mut tokens.tokens);
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
        let mut tokens = tokenize("1 + 2 - 3".to_string()).unwrap().tokens;
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
        let node = expr(&mut tokens.tokens);
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
        let node = expr(&mut tokens.tokens);
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
        let node = expr(&mut tokens.tokens);
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
