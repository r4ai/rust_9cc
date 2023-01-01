mod result;
mod tokenize;

use tokenize::{tokenize, Token, TokenKind};

fn expect_number(tk: &Option<Token>) -> i64 {
    match tk {
        Some(v) => match v.kind {
            TokenKind::Num => v.val,
            _ => panic!("数ではありません"),
        },
        None => panic!("数ではありません"),
    }
}

pub fn cli(args: Vec<String>) -> String {
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let mut tokens = tokenize(args[1].chars()).unwrap();
    let mut result = String::new();

    result.push_str(".intel_syntax noprefix\n");
    result.push_str(".globl main\n");
    result.push_str("main:\n");

    while let Some(token) = tokens.pop_front() {
        match token.kind {
            TokenKind::Num => {
                result.push_str(format!("  mov rax, {}\n", expect_number(&Some(token))).as_str());
            }
            TokenKind::Reserved => {
                if token.char == '+' {
                    result.push_str(
                        format!("  add rax, {}\n", expect_number(&tokens.pop_front())).as_str(),
                    );
                } else {
                    result.push_str(
                        format!("  sub rax, {}\n", expect_number(&tokens.pop_front())).as_str(),
                    );
                }
            }
            _ => panic!("予期しないトークン: {}", token.char),
        }
    }

    result.push_str("  ret\n");
    return result;
}
