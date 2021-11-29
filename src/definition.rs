use super::*;

#[derive(Debug, Clone)]
pub struct Definition {
    lhs: Function<Structure<Var>>,
    rhs: Expressions,
}

impl Definition {
    pub fn parse(p: &mut Parser) -> Result<Self, ParserError> {
        let mut q = p.clone();

        let _ = q.skip_many_space()?;
        let l = Function::<Structure<Var>>::parse(&mut q)?;
        let _ = q.skip_many_space()?;
        let _ = q.skip_char('=')?;
        let _ = q.skip_many_space()?;
        let r = Expressions::parse(&mut q)?;
        let _ = q.skip_many_space()?;
        let _ = q.test_eoi()?;

        *p = q;
        Ok(Self { lhs: l, rhs: r })
    }

    pub fn apply(&self, f: &Function<Structure<Fix>>) -> Option<Vec<ExprFix>> {
        check(self.lhs.language() == f.language())?;
        check(self.lhs.features().unify(f.features()).is_some())?;
        let p = self.lhs.argument();
        let s = f.argument();
        let ctx = s.unify(p)?;
        let mut exs = vec![];
        for e in &self.rhs.0 {
            exs.push(match e {
                Expression::Lit(x) => ExprFix::Lit(x.clone()),
                Expression::Fun(x) => ExprFix::Fun(x.replace(&ctx)?),
            });
        }
        Some(exs)
    }
}

impl std::fmt::Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.lhs, self.rhs)
    }
}

#[derive(Debug, Clone)]
pub enum ExprFix {
    Lit(Literal),
    Fun(Function<Structure<Fix>>)
}

impl std::fmt::Display for ExprFix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lit(x) => write!(f, "{}", x),
            Self::Fun(x) => write!(f, "{}", x),
        }
    }
}
