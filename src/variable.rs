use super::*;
use colored::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Variable(pub usize);

impl Parse for Variable {
    fn parse(p: &mut Parser) -> Result<Self, ParserError> {
        let mut q = p.clone();
        let _ = q.take_char(|c| *c == '$')?;
        let n = q.take_some(|c| c.is_ascii_digit())?;
        if let Ok(n) = n.parse() {
            *p = q;
            return Ok(Self(n));
        }
        q.fail("expecting variable")
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("${}", self.0).magenta())
    }
}
