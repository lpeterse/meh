#[derive(Debug, Clone)]
pub struct Parser<'a> {
    input: &'a str,
    state: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            state: input,
        }
    }

    pub fn fail<T, E: std::fmt::Display + 'static>(&self, expected: E) -> Result<T, ParserError> {
        let n = self.input.len();
        let m = self.state.len();
        Err(ParserError {
            position: n - m,
            expected: Box::new(expected),
        })
    }

    pub fn skip_char(&mut self, c: char) -> Result<(), ParserError> {
        if let Some(x) = self.state.strip_prefix(c) {
            self.state = x;
            Ok(())
        } else {
            self.fail("expecting char")
        }
    }

    pub fn skip_str(&mut self, s: &str) -> Result<(), ParserError> {
        if let Some(x) = self.state.strip_prefix(s) {
            self.state = x;
            Ok(())
        } else {
            self.fail("expecting string")
        }
    }

    pub fn skip_many_space(&mut self) -> Result<(), ParserError> {
        self.state = self.state.trim_start();
        Ok(())
    }

    pub fn take_many<P: FnMut(&char) -> bool>(&mut self, pred: P) -> Result<String, ParserError> {
        let cs = self.state.chars();
        let x: String = cs.take_while(pred).collect();
        self.state = &self.state[x.len()..];
        Ok(x)
    }

    pub fn take_some<P: FnMut(&char) -> bool>(&mut self, pred: P) -> Result<String, ParserError> {
        let cs = self.state.chars();
        let x: String = cs.take_while(pred).collect();
        if !x.is_empty() {
            self.state = &self.state[x.len()..];
            return Ok(x);
        }
        self.fail("expecting non-empty string")
    }

    pub fn take_char<P: FnOnce(&char) -> bool>(&mut self, pred: P) -> Result<char, ParserError> {
        let mut cs = self.state.chars();
        if let Some(c) = cs.next() {
            if pred(&c) {
                self.state = cs.as_str();
                return Ok(c);
            }
        }
        self.fail("expecting char")
    }

    pub fn take_ident_lower(&mut self) -> Result<String, ParserError> {
        let mut id = String::new();
        id.push(self.take_char(|c| c.is_ascii_lowercase())?);
        id.extend(
            self.take_many(|c| c.is_ascii_lowercase() || c.is_ascii_digit())?
                .chars(),
        );
        Ok(id)
    }

    pub fn take_ident_upper(&mut self) -> Result<String, ParserError> {
        let mut id = String::new();
        id.push(self.take_char(|c| c.is_ascii_uppercase())?);
        id.extend(
            self.take_many(|c| c.is_ascii_uppercase() || c.is_ascii_digit())?
                .chars(),
        );
        Ok(id)
    }

    pub fn test_eoi(&self) -> Result<(), ParserError> {
        if self.state.is_empty() {
            Ok(())
        } else {
            self.fail("expecting end of input")
        }
    }
}

pub struct ParserError {
    position: usize,
    expected: Box<dyn std::fmt::Display>,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Debug for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "at position {}: expected {}",
            self.position, self.expected
        )
    }
}

impl std::error::Error for ParserError {}

pub trait Parse: Sized {
    fn parse(p: &mut Parser) -> Result<Self, ParserError>;
}
