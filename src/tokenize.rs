use std::{collections::VecDeque, str::Chars};

use crate::result::{TokenizeError, TokenizeResult};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Reserved,
    Num,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub val: i64,
    pub char: char,
}

impl Token {
    pub fn new_op(c: char) -> TokenizeResult<Token> {
        if ['+', '-', '*', '/', '(', ')'].contains(&c) {
            Ok(Self {
                kind: TokenKind::Reserved,
                val: 0,
                char: c,
            })
        } else {
            Err(TokenizeError::InvalidOperator(c))
        }
    }

    pub fn new_num(val: i64) -> TokenizeResult<Token> {
        Ok(Self {
            kind: TokenKind::Num,
            val,
            char: ' ',
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tokens {
    pub user_input: String,
    pub tokens: VecDeque<Token>,
}

impl Tokens {
    pub fn init(user_input: String, capasity: usize) -> Self {
        Self {
            user_input,
            tokens: VecDeque::with_capacity(capasity),
        }
    }

    pub fn push_front(&mut self, token: Token) {
        self.tokens.push_front(token)
    }

    pub fn push_back(&mut self, token: Token) {
        self.tokens.push_back(token)
    }

    pub fn pop_front(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<Token> {
        self.tokens.pop_back()
    }

    pub fn consume_op(&mut self, op: char) -> bool {
        let token = match self.tokens.front() {
            Some(v) => v,
            None => return false,
        };
        if token.kind == TokenKind::Reserved && token.char == op {
            self.tokens.pop_front();
            return true;
        }
        false
    }
}

pub fn tokenize(input: String) -> TokenizeResult<Tokens> {
    let c = input.chars();
    fn check_tmp(tmp: &mut String, tokens: &mut Tokens) -> TokenizeResult<()> {
        if !tmp.is_empty() {
            let token = Token::new_num(match tmp.parse::<i64>() {
                Ok(val) => val,
                Err(_) => return Err(TokenizeError::InvalidNumber(tmp.clone())),
            })?;
            tokens.push_back(token);
            tmp.clear();
        }
        Ok(())
    }

    let mut tokens = Tokens::init(input.clone(), c.clone().count());
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
        if Token::new_op(c_i).is_ok() {
            check_tmp(&mut tmp, &mut tokens)?;
            let token = Token::new_op(c_i)?;
            tokens.push_back(token);
            continue;
        }
        return Err(TokenizeError::InvalidSyntax(c_i.to_string()));
    }
    check_tmp(&mut tmp, &mut tokens)?;

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn without_spaces() {
        let result = tokenize("1+2-300".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].char, '+');
        assert_eq!(result[2].val, 2);
        assert_eq!(result[3].char, '-');
        assert_eq!(result[4].val, 300);
    }

    #[test]
    fn with_spaces() {
        let result = tokenize("1 + 2 - 300".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].char, '+');
        assert_eq!(result[2].val, 2);
        assert_eq!(result[3].char, '-');
        assert_eq!(result[4].val, 300);
    }

    #[test]
    fn with_many_spaces() {
        let reuslt = tokenize(" 1   + 2 -300    ".to_string()).unwrap().tokens;
        assert_eq!(reuslt.len(), 5);
        assert_eq!(reuslt[0].val, 1);
        assert_eq!(reuslt[1].char, '+');
        assert_eq!(reuslt[2].val, 2);
        assert_eq!(reuslt[3].char, '-');
        assert_eq!(reuslt[4].val, 300);
    }

    #[test]
    fn multipy_with_spaces() {
        let result = tokenize("1 * 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].char, '*');
        assert_eq!(result[2].val, 2);
    }

    #[test]
    fn divide_with_spaces() {
        let result = tokenize("1 / 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].char, '/');
        assert_eq!(result[2].val, 2);
    }

    #[test]
    fn unary_operator_with_spaces() {
        let result = tokenize("-1 + 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].char, '-');
        assert_eq!(result[1].val, 1);
        assert_eq!(result[2].char, '+');
        assert_eq!(result[3].val, 2);
    }

    #[test]
    fn parenthesis_with_spaces() {
        let result = tokenize("(1 + 2) * 3".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 7);
        assert_eq!(result[0].char, '(');
        assert_eq!(result[1].val, 1);
        assert_eq!(result[2].char, '+');
        assert_eq!(result[3].val, 2);
        assert_eq!(result[4].char, ')');
        assert_eq!(result[5].char, '*');
        assert_eq!(result[6].val, 3);
    }

    #[test]
    fn invalid_number() {
        let result = tokenize("1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 0 + a".to_string());
        assert!(
            result.is_err(),
            "エラーが発生すべきですが、発生しませんでした。\n{:?}",
            result
        );
    }

    #[test]
    fn invalid_operator() {
        let result = tokenize("1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 0 % 4".to_string());
        assert!(
            result.is_err(),
            "エラーが発生すべきですが、発生しませんでした。\n{:?}",
            result
        );
    }
}
