mod result;
mod tokenize;

use std::string::ToString;
use std::{env, str::Chars};

use result::{TokenizeError, TokenizeResult};
use tokenize::{tokenize, Token, TokenKind};

fn expect_number(tk: &Token) -> i64 {
    match tk.kind {
        TokenKind::Num => tk.val,
        _ => panic!("数ではありません"),
    }
}

pub fn cli(args: Vec<String>) -> String {
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let tokens = tokenize(args[1].chars()).unwrap();
    let mut result = String::new();

    result.push_str(".intel_syntax noprefix\n");
    result.push_str(".globl main\n");
    result.push_str("main:\n");

    let mut do_skip = false;

    for (i, token) in tokens.iter().enumerate() {
        if i == 0 {
            result.push_str(format!("  mov rax, {}\n", expect_number(token)).as_str());
            continue;
        } else {
            if do_skip {
                do_skip = false;
                continue;
            }
            match token.kind {
                TokenKind::Reserved => {
                    if i == tokens.len() - 1 {
                        panic!("予期しないトークン: {}", token.char);
                    }
                    if token.char == '+' {
                        do_skip = true;
                        result.push_str(
                            format!("  add rax, {}\n", expect_number(&tokens[i + 1])).as_str(),
                        );
                        continue;
                    } else {
                        do_skip = true;
                        result.push_str(
                            format!("  sub rax, {}\n", expect_number(&tokens[i + 1])).as_str(),
                        );
                        continue;
                    }
                }
                _ => panic!("予期しないトークン: {}", token.char),
            }
        }
    }
    result.push_str("  ret\n");
    return result;
}
