use super::*;

#[derive(Debug, Clone)]
pub struct Expressions(pub Vec<Expression>);

impl Expressions {
    pub fn parse(p: &mut Parser) -> Result<Self, ParserError> {
        let mut es = vec![];
        p.skip_many_space()?;
        while let Ok(e) = Expression::parse(p) {
            p.skip_many_space()?;
            es.push(e);
        }
        Ok(Self(es))
    }
}

impl std::fmt::Display for Expressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in &self.0 {
            write!(f, "{} ", e)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Lit(Literal),
    Fun(Function<Variable>),
}

impl Expression {
    pub fn parse(p: &mut Parser) -> Result<Self, ParserError> {
        if let Ok(x) = Literal::parse(p) {
            return Ok(Self::Lit(x));
        }
        if let Ok(x) = Function::<Variable>::parse(p) {
            return Ok(Self::Fun(x));
        }
        p.fail("expecting expression")
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lit(x) => write!(f, "{}", x)?,
            Self::Fun(x) => write!(f, "{}", x)?,
        }
        Ok(())
    }
}
