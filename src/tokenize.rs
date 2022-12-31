use std::str::Chars;

use crate::result::{TokenizeError, TokenizeResult};

#[derive(Debug)]
pub enum TokenKind {
    Reserved,
    Num,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub val: i64,
    pub char: char,
}

impl Token {
    pub fn new_op(c: char) -> TokenizeResult<Token> {
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

    pub fn new_num(val: i64) -> TokenizeResult<Token> {
        Ok(Self {
            kind: TokenKind::Num,
            val,
            char: ' ',
        })
    }
}

pub fn tokenize(c: Chars) -> TokenizeResult<Vec<Token>> {
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

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn without_spaces() {
        let result = tokenize("1+2-300".chars()).unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].char, '+');
        assert_eq!(result[2].val, 2);
        assert_eq!(result[3].char, '-');
        assert_eq!(result[4].val, 300);
    }

    #[test]
    fn with_spaces() {
        let result = tokenize("1 + 2 - 300".chars()).unwrap();
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].char, '+');
        assert_eq!(result[2].val, 2);
        assert_eq!(result[3].char, '-');
        assert_eq!(result[4].val, 300);
    }

    #[test]
    fn with_many_spaces() {
        let reuslt = tokenize(" 1   + 2 -300    ".chars()).unwrap();
        assert_eq!(reuslt.len(), 5);
        assert_eq!(reuslt[0].val, 1);
        assert_eq!(reuslt[1].char, '+');
        assert_eq!(reuslt[2].val, 2);
        assert_eq!(reuslt[3].char, '-');
        assert_eq!(reuslt[4].val, 300);
    }
}
