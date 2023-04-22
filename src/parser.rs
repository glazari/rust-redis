use crate::datastore::Command;

#[derive(Debug, PartialEq)]
enum Token {
    Set,
    Get,
    Value(String),
}

impl Token {
    #[cfg(test)]
    fn value(value: &str) -> Token {
        Token::Value(value.to_string())
    }
}

struct Tokenizer {
    input: String,
    pos: usize,
}

impl Tokenizer {
    fn new(input: String) -> Tokenizer {
        Tokenizer { input, pos: 0 }
    }

    fn next_token(&mut self) -> Option<Token> {
        let token: Token;

        if self.pos >= self.input.len() {
            return None;
        }

        // skip whitespace
        while self.input.chars().nth(self.pos).is_some()
            && self.input.chars().nth(self.pos).unwrap().is_whitespace()
        {
            self.pos += 1;
        }

        // while not whitespace
        let mut end = self.pos;
        while self.input.chars().nth(end).is_some()
            && !self.input.chars().nth(end).unwrap().is_whitespace()
        {
            end += 1;
        }

        let token_str = &self.input[self.pos..end];
        self.pos = end;

        token = match token_str {
            "set" => Token::Set,
            "get" => Token::Get,
            "" => return None,
            _ => Token::Value(token_str.to_string()),
        };

        Some(token)
    }

    #[cfg(test)]
    fn get_all(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
}

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            tokenizer: Tokenizer::new(input),
        }
    }

    pub fn parse(&mut self) -> Result<Command, String> {
        let token = self.tokenizer.next_token();
        match token {
            Some(Token::Set) => {
                let key = match self.tokenizer.next_token() {
                    Some(Token::Value(key)) => key,
                    Some(token) => Err(format!("Set Expected key, got {:?}", token))?,
                    None => Err("Set Expected key".to_string())?,
                };
                let value = match self.tokenizer.next_token() {
                    Some(Token::Value(value)) => value,
                    Some(token) => Err(format!("Set Expected value, got {:?}", token))?,
                    None => Err("Set Expected value".to_string())?,
                };
                Ok(Command::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                })
            }
            Some(Token::Get) => {
                let key = match self.tokenizer.next_token() {
                    Some(Token::Value(key)) => key,
                    Some(token) => Err(format!("Get Expected key, got {:?}", token))?,
                    None => Err("Get Expected key".to_string())?,
                };
                Ok(Command::Get {
                    key: key.to_string(),
                })
            }
            _ => Err("Unknown command".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parses() {
        let tests = vec![
            ("set foo bar", Ok(Command::set("foo", "bar"))),
            ("get foo", Ok(Command::get("foo"))),
            ("set foo   bar", Ok(Command::set("foo", "bar"))),
            ("get     bar", Ok(Command::get("bar"))),
            ("not a command", Err("Unknown command".to_string())),
            ("set", Err("Set Expected key".to_string())),
            ("set foo", Err("Set Expected value".to_string())),
            ("get", Err("Get Expected key".to_string())),
            ("set foo ", Err("Set Expected value".to_string())),
        ];

        for (input, expected) in tests {
            let mut parser = Parser::new(input.to_string());
            let cmd = parser.parse();
            assert_eq!(expected, cmd);
        }
    }

    #[test]
    fn tokenizer() {
        let tests = vec![
            (
                "set foo bar",
                vec![Token::Set, Token::value("foo"), Token::value("bar")],
            ),
            ("get foo", vec![Token::Get, Token::value("foo")]),
            (
                "set foo   bar",
                vec![Token::Set, Token::value("foo"), Token::value("bar")],
            ),
            ("get     bar", vec![Token::Get, Token::value("bar")]),
            ("set foo ", vec![Token::Set, Token::value("foo")]),
        ];

        for (input, expected) in tests {
            let mut tokenizer = Tokenizer::new(input.to_string());
            let tokens = tokenizer.get_all();
            assert_eq!(expected, tokens);
        }
    }
}
