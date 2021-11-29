use super::*;
use std::collections::HashMap;
use colored::*;

#[derive(Debug, Clone)]
pub struct Function<T> {
    language: Language,
    features: Features,
    argument: T,
}

impl<T> Function<T> {
    pub fn language(&self) -> Language {
        self.language
    }

    pub fn features(&self) -> &Features {
        &self.features
    }

    pub fn argument(&self) -> &T {
        &self.argument
    }
}

impl<T: Parse> Parse for Function<T> {
    fn parse(p: &mut Parser) -> Result<Self, ParserError> {
        let mut q = p.clone();

        let _ = q.skip_many_space()?;
        let l = Language::parse(&mut q)?;
        let f = Features::parse(&mut q)?;
        let _ = q.skip_char('(')?;
        let _ = q.skip_many_space()?;
        let a = T::parse(&mut q)?;
        let _ = q.skip_many_space()?;
        let _ = q.skip_char(')')?;

        *p = q;
        Ok(Self {
            language: l,
            features: f,
            argument: a,
        })
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Function<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.language.to_string().yellow().dimmed())?;
        if self.features != Features::default() {
            write!(
                f,
                "{}{}{}",
                "<".yellow().dimmed(),
                self.features,
                ">".yellow().dimmed()
            )?;
        }
        write!(
            f,
            "{}{}{}",
            "(".yellow().dimmed(),
            self.argument,
            ")".yellow().dimmed()
        )?;
        Ok(())
    }
}

impl Function<Variable> {
    pub fn replace(
        &self,
        ctx: &HashMap<Variable, Structure<Fix>>,
    ) -> Option<Function<Structure<Fix>>> {
        let argument = ctx.get(&self.argument)?.clone();
        Some(Function {
            language: self.language,
            features: self.features,
            argument,
        })
    }
}
