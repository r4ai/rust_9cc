use std::collections::VecDeque;

use crate::result::{TokenizeError, TokenizeResult};

#[derive(Debug, PartialEq, Eq)]
pub struct UserInput {
    chars: VecDeque<char>,
}

impl UserInput {
    pub fn new(input: String) -> Self {
        Self {
            chars: input.chars().collect(),
        }
    }

    pub fn front(&self) -> Option<&char> {
        self.chars.front()
    }

    pub fn pop_front(&mut self) -> Option<char> {
        self.chars.pop_front()
    }

    fn get(&self, index: usize) -> Option<&char> {
        self.chars.get(index)
    }

    pub fn parse_num(&mut self) -> Option<usize> {
        if !self.front()?.is_ascii_digit() {
            return None;
        }

        let mut num = 0;
        while let Some(c) = self.front() {
            if !c.is_ascii_digit() {
                break;
            }
            num = num * 10 + (c.to_digit(10).unwrap() as usize);
            self.pop_front();
        }
        Some(num)
    }

    pub fn parse_op(&mut self) -> Option<String> {
        match self.front()? {
            '=' => match self.get(1)? {
                '=' => {
                    self.pop_front()?;
                    self.pop_front()?;
                    Some("==".to_string())
                }
                _ => {
                    self.pop_front()?;
                    Some("=".to_string())
                }
            },
            '!' => match self.get(1)? {
                '=' => {
                    self.pop_front()?;
                    self.pop_front()?;
                    Some("!=".to_string())
                }
                _ => None,
            },
            '<' => match self.get(1)? {
                '=' => {
                    self.pop_front()?;
                    self.pop_front()?;
                    Some("<=".to_string())
                }
                _ => {
                    self.pop_front()?;
                    Some("<".to_string())
                }
            },
            '>' => match self.get(1)? {
                '=' => {
                    self.pop_front()?;
                    self.pop_front()?;
                    Some(">=".to_string())
                }
                _ => {
                    self.pop_front()?;
                    Some(">".to_string())
                }
            },
            '+' | '-' | '*' | '/' | '(' | ')' => {
                let c = self.pop_front()?;
                Some(c.to_string())
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Reserved,
    Num,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub val: i64,
    pub str: String,
    pub len: usize,
}

impl Token {
    pub fn new_op(c: String) -> TokenizeResult<Token> {
        Ok(Self {
            kind: TokenKind::Reserved,
            val: 0,
            str: c,
            len: 1,
        })
    }

    pub fn new_num(val: i64) -> TokenizeResult<Token> {
        Ok(Self {
            kind: TokenKind::Num,
            val,
            str: " ".to_string(),
            len: 1,
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

    pub fn push_back(&mut self, token: Token) {
        self.tokens.push_back(token)
    }

    pub fn pop_front(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn front(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn consume_op(&mut self, op: &str) -> bool {
        let token = match self.front() {
            Some(v) => v,
            None => return false,
        };
        if token.kind == TokenKind::Reserved && token.str == op {
            self.pop_front();
            return true;
        }
        false
    }
}

pub fn tokenize(input: String) -> TokenizeResult<Tokens> {
    let mut user_input = UserInput::new(input.clone());
    let mut tokens = Tokens::init(input.clone(), input.capacity());

    while let Some(c) = user_input.front() {
        if c.is_ascii_whitespace() {
            user_input.pop_front();
            continue;
        }

        if let Some(op) = user_input.parse_op() {
            tokens.push_back(Token::new_op(op)?);
            continue;
        }

        if let Some(num) = user_input.parse_num() {
            tokens.push_back(Token::new_num(num as i64)?);
            continue;
        }

        return Err(TokenizeError::InvalidSyntax(input));
    }

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
        assert_eq!(result[1].str, "+");
        assert_eq!(result[2].val, 2);
        assert_eq!(result[3].str, "-");
        assert_eq!(result[4].val, 300);
    }

    #[test]
    fn with_spaces() {
        let result = tokenize("1 + 2 - 300".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].str, "+");
        assert_eq!(result[2].val, 2);
        assert_eq!(result[3].str, "-");
        assert_eq!(result[4].val, 300);
    }

    #[test]
    fn with_many_spaces() {
        let reuslt = tokenize(" 1   + 2 -300    ".to_string()).unwrap().tokens;
        assert_eq!(reuslt.len(), 5);
        assert_eq!(reuslt[0].val, 1);
        assert_eq!(reuslt[1].str, "+");
        assert_eq!(reuslt[2].val, 2);
        assert_eq!(reuslt[3].str, "-");
        assert_eq!(reuslt[4].val, 300);
    }

    #[test]
    fn multipy_with_spaces() {
        let result = tokenize("1 * 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].str, "*");
        assert_eq!(result[2].val, 2);
    }

    #[test]
    fn divide_with_spaces() {
        let result = tokenize("1 / 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].str, "/");
        assert_eq!(result[2].val, 2);
    }

    #[test]
    fn unary_operator_with_spaces() {
        let result = tokenize("-1 + 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].str, "-");
        assert_eq!(result[1].val, 1);
        assert_eq!(result[2].str, "+");
        assert_eq!(result[3].val, 2);
    }

    #[test]
    fn parenthesis_with_spaces() {
        let result = tokenize("(1 + 2) * 3".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 7);
        assert_eq!(result[0].str, "(");
        assert_eq!(result[1].val, 1);
        assert_eq!(result[2].str, "+");
        assert_eq!(result[3].val, 2);
        assert_eq!(result[4].str, ")");
        assert_eq!(result[5].str, "*");
        assert_eq!(result[6].val, 3);
    }

    #[test]
    fn leq_operator_with_spaces() {
        let result = tokenize("1 <= 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].str, "<=");
        assert_eq!(result[2].val, 2);
    }

    #[test]
    fn lt_operator_with_spaces() {
        let result = tokenize("1 < 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].str, "<");
        assert_eq!(result[2].val, 2);
    }

    #[test]
    fn geq_operator_with_spaces() {
        let result = tokenize("1 >= 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].str, ">=");
        assert_eq!(result[2].val, 2);
    }

    #[test]
    fn gt_operator_with_spaces() {
        let result = tokenize("1 > 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].str, ">");
        assert_eq!(result[2].val, 2);
    }

    #[test]
    fn ne_operator_with_spaces() {
        let result = tokenize("1 != 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].str, "!=");
        assert_eq!(result[2].val, 2);
    }

    #[test]
    fn eq_operator_with_spaces() {
        let result = tokenize("1 == 2".to_string()).unwrap().tokens;
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].str, "==");
        assert_eq!(result[2].val, 2);
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
