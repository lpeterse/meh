use super::*;
use colored::*;
use std::io::BufRead;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Database {
    pub definitions: Vec<Definition>,
}

impl Database {
    pub fn load(path: &str) -> Result<Arc<Self>, Box<dyn std::error::Error>> {
        let f = std::fs::File::open(path)?;
        let lines = std::io::BufReader::new(f).lines();
        let mut eqs = vec![];

        for line in lines {
            let mut line = line?;
            if line.trim().is_empty() || line.trim().starts_with('#') {
                continue;
            }
            let mut p = Parser::new(&mut line);
            match Definition::parse(&mut p) {
                Ok(x) => {
                    println!("{}", x);
                    eqs.push(x);
                }
                Err(e) => {
                    eprintln!("{}", format!("{}", e).red());
                    return Err("".into());
                }
            }
        }

        Ok(Arc::new(Self { definitions: eqs }))
    }
}
