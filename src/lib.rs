mod codegen;
mod parse;
mod result;
mod tokenize;

use codegen::gen;
use parse::expr;
use tokenize::tokenize;

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
    // dbg!(&node);

    let asm_code = gen(&node);
    result.push_str(asm_code.as_str());

    result.push_str("  pop rax\n");
    result.push_str("  ret\n");
    return result;
}

#[cfg(test)]
mod tests {
    use super::{expr, tokenize};
    use crate::parse::{Node, NodeKind};

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
