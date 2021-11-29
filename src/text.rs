use super::*;

#[derive(Debug, Clone)]
pub struct Text(pub Vec<Literal>);

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.0 {
            write!(f, "{} ", i)?
        }
        Ok(())
    }
}
