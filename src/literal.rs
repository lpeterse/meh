use super::*;
use colored::*;

#[derive(Debug, Clone)]
pub struct Literal(String);

impl Parse for Literal {
    fn parse(p: &mut Parser) -> Result<Self, ParserError> {
        let mut q = p.clone();

        q.take_char(|c| c == &'"')?;
        let s = q.take_many(|c| c != &'"')?;
        q.take_char(|c| c == &'"')?;

        *p = q;
        Ok(Self(s))
    }
}

impl From<&str> for Literal {
    fn from(x: &str) -> Self {
        Self(x.into())
    }
}

impl AsRef<str> for Literal {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self.0).yellow())
    }
}
