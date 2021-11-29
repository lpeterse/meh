use super::*;
use std::collections::HashMap;
use colored::*;

#[derive(Debug, Clone)]
pub struct Structure<T> {
    name: String,
    args: HashMap<String, T>,
}

impl Structure<Fix> {
    pub fn unify(&self, pattern: &Structure<Var>) -> Option<HashMap<Variable, Structure<Fix>>> {
        let mut ctx = HashMap::default();
        if self.unify_(pattern, &mut ctx).is_some() {
            Some(ctx)
        } else {
            None
        }
    }

    fn unify_(
        &self,
        pattern: &Structure<Var>,
        ctx: &mut HashMap<Variable, Structure<Fix>>,
    ) -> Option<()> {
        if self.name == pattern.name && self.args.len() == pattern.args.len() {
            for (k, Fix(s)) in &self.args {
                match pattern.args.get(k)? {
                    Var::S(x) => s.unify_(x, ctx)?,
                    Var::V(x) => ctx.insert(*x, s.clone()).map(drop).unwrap_or_default(),
                }
            }
            Some(())
        } else {
            None
        }
    }
}

impl<T: Parse> Parse for Structure<T> {
    fn parse(p: &mut Parser) -> Result<Self, ParserError> {
        let mut q = p.clone();
        let name = q.take_ident_upper()?;
        let mut x = Self {
            name,
            args: HashMap::default(),
        };
        if let Ok(_) = q.take_char(|c| *c == '[') {
            loop {
                let _ = q.skip_many_space()?;
                let k = q.take_ident_lower()?;
                let _ = q.skip_many_space()?;
                let _ = q.take_char(|c| *c == ':')?;
                let _ = q.skip_many_space()?;
                let v = T::parse(&mut q)?;
                if x.args.insert(k, v).is_some() {
                    return q.fail("duplicate key");
                }
                let _ = q.skip_many_space()?;
                if q.take_char(|c| *c == ']').is_ok() {
                    break;
                }
                q.take_char(|c| *c == ',')?;
            }
        }
        *p = q;
        Ok(x)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Structure<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.bright_cyan())?;
        write!(f, "{}", "[".cyan().dimmed())?;
        let mut kvs = self.args.iter();
        if let Some((k, v)) = kvs.next() {
            write!(f, "{}{} {}", k.cyan().dimmed(), ":".cyan().dimmed(), v)?;
        }
        for (k, v) in kvs {
            write!(
                f,
                "{} {}{} {}",
                ",".cyan().dimmed(),
                k.cyan().dimmed(),
                ":".cyan().dimmed(),
                v
            )?;
        }
        write!(f, "{}", "]".cyan().dimmed())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Fix(Structure<Fix>);

impl From<Structure<Fix>> for Fix {
    fn from(x: Structure<Fix>) -> Self {
        Self(x)
    }
}

impl Parse for Fix {
    fn parse(p: &mut Parser) -> Result<Self, ParserError> {
        Ok(Self(Structure::<Fix>::parse(p)?))
    }
}

#[derive(Debug, Clone)]
pub enum Var {
    V(Variable),
    S(Structure<Var>),
}

impl Parse for Var {
    fn parse(p: &mut Parser) -> Result<Self, ParserError> {
        if let Ok(x) = Variable::parse(p) {
            return Ok(Self::V(x));
        }
        if let Ok(x) = Structure::<Var>::parse(p) {
            return Ok(Self::S(x));
        }
        p.fail("expecting Foo|Enm|Var")
    }
}

impl std::fmt::Display for Fix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V(x) => write!(f, "{}", x),
            Self::S(x) => write!(f, "{}", x),
        }
    }
}
