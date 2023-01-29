use std::collections::VecDeque;

use crate::{
    parse::LVars,
    result::{TokenizeError, TokenizeResult},
};

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
            ';' => {
                let c = self.pop_front()?;
                Some(c.to_string())
            }
            _ => None,
        }
    }

    pub fn parse_lvar(&mut self) -> Option<String> {
        if !self.front()?.is_ascii_alphabetic() {
            return None;
        }

        let mut lvar = String::new();
        while let Some(c) = self.front() {
            if c.is_ascii_alphanumeric() {
                lvar.push(*c);
                self.pop_front();
            } else {
                break;
            }
        }
        Some(lvar)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Reserved, // 記号
    Ident,    // 識別子
    Num,      // 整数トークン
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
            str: c.clone(),
            len: c.len(),
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

    pub fn new_lvar(lvar: String) -> TokenizeResult<Token> {
        Ok(Self {
            kind: TokenKind::Ident,
            val: 0,
            str: lvar.clone(),
            len: lvar.len(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tokens {
    pub user_input: String,
    pub lvars: LVars,
    pub tokens: VecDeque<Token>,
}

impl Tokens {
    pub fn init(user_input: String, capasity: usize) -> Self {
        Self {
            user_input,
            lvars: LVars::new(),
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

    pub fn len(&self) -> usize {
        self.tokens.len()
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

        if let Some(lvar) = user_input.parse_lvar() {
            tokens.push_back(Token::new_lvar(lvar)?);
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
mod tests_userinput {
    use super::UserInput;

    #[test]
    fn parse_num() {
        let mut user_input = UserInput::new("123".to_string());
        assert_eq!(user_input.parse_num(), Some(123));
        assert_eq!(user_input.parse_num(), None);
    }

    #[test]
    fn parse_num_to_be_skipped() {
        let mut user_input_op = UserInput::new("+".to_string());
        let mut user_input_lvar = UserInput::new("abc".to_string());
        assert_eq!(user_input_op.parse_num(), None);
        assert_eq!(user_input_lvar.parse_num(), None);
    }

    #[test]
    fn parse_op_single() {
        let mut user_input = UserInput::new("+".to_string());
        assert_eq!(user_input.parse_op(), Some("+".to_string()));
        assert_eq!(user_input.parse_op(), None);
    }

    #[test]
    fn parse_op_double() {
        let mut user_input = UserInput::new("<=".to_string());
        assert_eq!(user_input.parse_op(), Some("<=".to_string()));
        assert_eq!(user_input.parse_op(), None);
    }

    #[test]
    fn parse_op_to_be_skipped() {
        let mut user_input_num = UserInput::new("3".to_string());
        let mut user_input_lvar = UserInput::new("abc".to_string());
        assert_eq!(user_input_num.parse_op(), None);
        assert_eq!(user_input_lvar.parse_op(), None);
    }

    #[test]
    fn parse_lvar() {
        let mut user_input = UserInput::new("abc".to_string());
        assert_eq!(user_input.parse_lvar(), Some("abc".to_string()));
        assert_eq!(user_input.parse_lvar(), None);
    }

    #[test]
    fn parse_lvar_to_be_skipped() {
        let mut user_input_num = UserInput::new("123".to_string());
        let mut user_input_op = UserInput::new("+".to_string());
        assert_eq!(user_input_num.parse_lvar(), None);
        assert_eq!(user_input_op.parse_lvar(), None);
    }
}

#[cfg(test)]
mod tests_token {
    use super::Token;
    use super::TokenKind;

    #[test]
    fn new_num() {
        let token = Token::new_num(123).unwrap();
        assert_eq!(token.kind, TokenKind::Num);
        assert_eq!(token.val, 123);
        assert_eq!(token.str, " ");
        assert_eq!(token.len, 1);
    }

    #[test]
    fn new_op() {
        let token = Token::new_op("+".to_string()).unwrap();
        assert_eq!(token.kind, TokenKind::Reserved);
        assert_eq!(token.val, 0);
        assert_eq!(token.str, "+");
        assert_eq!(token.len, 1);
    }

    #[test]
    fn new_lvar() {
        let token = Token::new_lvar("abc".to_string()).unwrap();
        assert_eq!(token.kind, TokenKind::Ident);
        assert_eq!(token.val, 0);
        assert_eq!(token.str, "abc");
        assert_eq!(token.len, 3);
    }
}

#[cfg(test)]
mod tests_tokenize {
    use crate::tokenize::TokenKind;

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
    fn single_local_variable() {
        let result = tokenize("a = 3 * -2; b = 2; -a + b;".to_string())
            .unwrap()
            .tokens;
        assert_eq!(result.len(), 16);
        assert_eq!(result[0].kind, TokenKind::Ident);
        assert_eq!(result[0].str, "a");
        assert_eq!(result[1].str, "=");
        assert_eq!(result[2].val, 3);
        assert_eq!(result[3].str, "*");
        assert_eq!(result[4].str, "-");
        assert_eq!(result[5].val, 2);
        assert_eq!(result[6].str, ";");
        assert_eq!(result[7].kind, TokenKind::Ident);
        assert_eq!(result[7].str, "b");
        assert_eq!(result[8].str, "=");
        assert_eq!(result[9].val, 2);
        assert_eq!(result[10].str, ";");
        assert_eq!(result[11].str, "-");
        assert_eq!(result[12].kind, TokenKind::Ident);
        assert_eq!(result[12].str, "a");
        assert_eq!(result[13].str, "+");
        assert_eq!(result[14].kind, TokenKind::Ident);
        assert_eq!(result[14].str, "b");
        assert_eq!(result[15].str, ";");
    }

    #[test]
    fn newline_operator() {
        let result = tokenize("1 + 3 * 4; -2 * 3 + 3".to_string())
            .unwrap()
            .tokens;
        assert_eq!(result.len(), 12);
        assert_eq!(result[0].val, 1);
        assert_eq!(result[1].str, "+");
        assert_eq!(result[2].val, 3);
        assert_eq!(result[3].str, "*");
        assert_eq!(result[4].val, 4);
        assert_eq!(result[5].str, ";");
        assert_eq!(result[6].str, "-");
        assert_eq!(result[7].val, 2);
        assert_eq!(result[8].str, "*");
        assert_eq!(result[9].val, 3);
        assert_eq!(result[10].str, "+");
        assert_eq!(result[11].val, 3);
    }

    #[test]
    fn invalid_operator() {
        let result = tokenize("1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 0 % 4".to_string());
        assert!(
            result.is_err(),
            "エラーが発生すべきですが、発生しませんでした。\n{result:?}"
        );
    }
}
