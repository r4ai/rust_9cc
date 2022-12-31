mod result;

use std::string::ToString;
use std::{env, str::Chars};

use result::{TokenizeError, TokenizeResult};

#[derive(Debug)]
enum TokenKind {
    Reserved,
    Num,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    val: i64,
    char: char,
}

impl Token {
    fn new_op(c: char) -> TokenizeResult<Token> {
        if c == '+' || c == '-' {
            Ok(Self {
                kind: TokenKind::Reserved,
                val: 0,
                char: c,
            })
        } else {
            Err(TokenizeError::InvalidOperator(c))
        }
    }

    fn new_num(val: i64) -> TokenizeResult<Token> {
        Ok(Self {
            kind: TokenKind::Num,
            val,
            char: ' ',
        })
    }
}

fn tokenize(c: Chars) -> TokenizeResult<Vec<Token>> {
    fn check_tmp(tmp: &mut String, tokens: &mut Vec<Token>) -> TokenizeResult<()> {
        if !tmp.is_empty() {
            let token = Token::new_num(match tmp.parse::<i64>() {
                Ok(val) => val,
                Err(_) => return Err(TokenizeError::InvalidNumber(tmp.clone())),
            })?;
            tokens.push(token);
            tmp.clear();
        }
        Ok(())
    }

    let mut tokens: Vec<Token> = vec![];
    let mut tmp = String::new();

    for c_i in c {
        if c_i == ' ' {
            check_tmp(&mut tmp, &mut tokens)?;
            continue;
        }
        if c_i.is_ascii_digit() {
            tmp.push(c_i);
            continue;
        }
        if c_i == '+' || c_i == '-' {
            check_tmp(&mut tmp, &mut tokens)?;
            let token = Token::new_op(c_i)?;
            tokens.push(token);
            continue;
        }
        return Err(TokenizeError::InvalidSyntax(c_i.to_string()));
    }
    check_tmp(&mut tmp, &mut tokens)?;

    Ok(tokens)
}

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
