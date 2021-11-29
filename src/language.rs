use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    De,
    En,
    No,
}

impl Language {
    const LS: [(Self, &'static str); 3] = [(Self::De, "DE"), (Self::En, "EN"), (Self::No, "NO")];

    pub fn parse(p: &mut Parser) -> Result<Self, ParserError> {
        for (a, b) in Self::LS {
            if p.skip_str(b).is_ok() {
                return Ok(a);
            }
        }
        p.fail("language code like DE|EN|NO")
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (a, b) in Self::LS {
            if a == *self {
                write!(f, "{}", b)?;
            }
        }
        Ok(())
    }
}
