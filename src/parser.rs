#[derive(Debug, PartialEq)]
enum Command {
    Set { key: String, value: String },
    Get { key: String },
}

impl Command {
    fn to_string(&self) -> String {
        match self {
            Command::Set { key, value } => format!("set {} {}", key, value),
            Command::Get { key } => format!("get {}", key),
        }
    }

    fn Set(key: &str, value: &str) -> Command {
        Command::Set {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    fn Get(key: &str) -> Command {
        Command::Get {
            key: key.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Set,
    Get,
    Value(String),
}

impl Token {
    fn to_string(&self) -> String {
        match self {
            Token::Set => "set".to_string(),
            Token::Get => "get".to_string(),
            Token::Value(value) => value.to_string(),
        }
    }

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
            _ => Token::Value(token_str.to_string()),
        };

        Some(token)
    }

    fn get_all(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
}

struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    fn new(input: String) -> Parser {
        Parser {
            tokenizer: Tokenizer::new(input),
        }
    }

    fn parse(&mut self) -> Command {
        let token = self.tokenizer.next_token();
        match token {
            Some(Token::Set) => {
                let key = match self.tokenizer.next_token().unwrap() {
                    Token::Value(key) => key,
                    _ => panic!("Expected key"),
                };
                let value = match self.tokenizer.next_token().unwrap() {
                    Token::Value(value) => value,
                    _ => panic!("Expected value"),
                };
                Command::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                }
            }
            Some(Token::Get) => {
                let key = match self.tokenizer.next_token().unwrap() {
                    Token::Value(key) => key,
                    _ => panic!("Expected key, got {:?}", token),
                };
                Command::Get {
                    key: key.to_string(),
                }
            }
            _ => panic!("Unknown command"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parses() {
        let tests = vec![
            ("set foo bar", Command::Set("foo", "bar")),
            ("get foo", Command::Get("foo")),
            ("set foo   bar", Command::Set("foo", "bar")),
            ("get     bar", Command::Get("bar")),
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
            ("set foo bar", vec![Token::Set, Token::value("foo"), Token::value("bar")]),
            ("get foo", vec![Token::Get, Token::value("foo")]),
            ("set foo   bar", vec![Token::Set, Token::value("foo"), Token::value("bar")]),
            ("get     bar", vec![Token::Get, Token::value("bar")]),
        ];

        for (input, expected) in tests {
            let mut tokenizer = Tokenizer::new(input.to_string());
            let tokens = tokenizer.get_all();
            assert_eq!(expected, tokens);
        }
            }
}
