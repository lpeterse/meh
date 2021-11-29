use super::*;
use colored::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Person {
    P1,
    P2,
    P3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Casus {
    Nom,
    Gen,
    Dat,
    Acc,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Genus {
    M,
    F,
    N,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Numerus {
    Sg,
    Pl,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Features {
    casus: Option<Casus>,
    person: Option<Person>,
    numerus: Option<Numerus>,
    genus: Option<Genus>,
}

impl Features {
    pub fn unify(&self, other: &Self) -> Option<Self> {
        Some(Self {
            casus: unify(&self.casus, &other.casus)?,
            person: unify(&self.person, &other.person)?,
            numerus: unify(&self.numerus, &other.numerus)?,
            genus: unify(&self.genus, &other.genus)?,
        })
    }

    pub fn parse(p: &mut Parser) -> Result<Self, ParserError> {
        let mut q = p.clone();
        if q.skip_char('<').is_ok() {
            let mut fs = Features::default();
            loop {
                q.skip_many_space()?;
                if let Ok(x) = q.take_ident_upper() {
                    match x.as_str() {
                        "S" => fs.numerus = Some(Numerus::Sg),
                        "P" => fs.numerus = Some(Numerus::Pl),
                        "M" => fs.genus = Some(Genus::M),
                        "F" => fs.genus = Some(Genus::F),
                        "N" => fs.genus = Some(Genus::N),
                        "NOM" => fs.casus = Some(Casus::Nom),
                        "GEN" => fs.casus = Some(Casus::Gen),
                        "DAT" => fs.casus = Some(Casus::Dat),
                        "ACC" => fs.casus = Some(Casus::Acc),
                        "P1" => fs.person = Some(Person::P1),
                        "P2" => fs.person = Some(Person::P2),
                        "P3" => fs.person = Some(Person::P3),
                        _ => return q.fail("expecting feature"),
                    }
                }
                if q.skip_char('>').is_ok() {
                    *p = q;
                    return Ok(fs);
                }
                q.skip_many_space()?;
                let _ = q.skip_char(',')?;
            }
        } else {
            Ok(Features::default())
        }
    }
}

impl std::fmt::Display for Features {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut xs = vec![];
        match self.casus {
            Some(Casus::Nom) => xs.push("NOM"),
            Some(Casus::Gen) => xs.push("GEN"),
            Some(Casus::Dat) => xs.push("DAT"),
            Some(Casus::Acc) => xs.push("ACC"),
            None => (),
        }
        match self.person {
            Some(Person::P1) => xs.push("P1"),
            Some(Person::P2) => xs.push("P2"),
            Some(Person::P3) => xs.push("P3"),
            None => (),
        }
        match self.numerus {
            Some(Numerus::Sg) => xs.push("S"),
            Some(Numerus::Pl) => xs.push("P"),
            None => (),
        }
        match self.genus {
            Some(Genus::M) => xs.push("M"),
            Some(Genus::F) => xs.push("F"),
            Some(Genus::N) => xs.push("N"),
            None => (),
        }
        let mut xs = xs.iter();
        if let Some(x) = xs.next() {
            write!(f, "{}", x.bright_black())?;
        }
        for x in xs {
            write!(f, "{}{}", ",".bright_black(), x.bright_black())?;
        }
        Ok(())
    }
}

fn unify<T: Copy + PartialEq>(x: &Option<T>, y: &Option<T>) -> Option<Option<T>> {
    match (x, y) {
        (Some(x), Some(y)) if x == y => Some(Some(*x)),
        (Some(x), None) => Some(Some(*x)),
        (None, Some(x)) => Some(Some(*x)),
        (None, None) => Some(None),
        _ => None,
    }
}
