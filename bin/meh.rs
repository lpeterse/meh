use colored::*;
use meh::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new()?;
    let _ = app.rl.load_history(App::HISTORY);
    let r = app.run();
    let _ = app.rl.save_history(App::HISTORY);
    r
}

struct App {
    db: Arc<Database>,
    rl: Editor<()>,
}

impl App {
    const DB: &'static str = "db/lang/de.meh";
    const PROMPT: &'static str = ">> ";
    const HISTORY: &'static str = ".meh_history";

    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = Database::load(Self::DB)?;
        let rl = Editor::<()>::new();
        Ok(Self { db, rl })
    }

    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            println!("");
            match self.rl.readline(Self::PROMPT) {
                Ok(line) => {
                    match line.as_str().trim() {
                        ":r" => self.cmd_reload()?,
                        line => self.cmd_interpret(line)?,
                    }
                    self.rl.add_history_entry(line.as_str());
                }
                Err(ReadlineError::Eof) => return Ok(()),
                Err(e) => println!("{}", e.to_string().red()),
            }
        }
    }

    fn cmd_reload(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.db = Database::load(Self::DB)?;
        Ok(())
    }

    fn cmd_interpret(&mut self, line: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut p = Parser::new(&line);
        match Function::<Structure<Fix>>::parse(&mut p) {
            Ok(f) => {
                println!("{}", f);
                println!("");
                for t in Translations::new(self.db.clone(), f) {
                    println!("{}", t);
                }
            }
            Err(e) => {
                println!("{}", e.to_string().red());
            }
        }
        Ok(())
    }
}
