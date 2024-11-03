use awesome_calc::{error::Result, lexer::Lexer, parser::Parser};
use std::io::{self, Write};

const SPLASH: &str = r#"
  /_\__      _____  ___  ___  _ __ ___   ___  / __\__ _| | ___
 //_\\ \ /\ / / _ \/ __|/ _ \| '_ ` _ \ / _ \/ /  / _` | |/ __|
/  _  \ V  V /  __/\__ \ (_) | | | | | |  __/ /__| (_| | | (__
\_/ \_/\_/\_/ \___||___/\___/|_| |_| |_|\___\____/\__,_|_|\___|

Welcome! v1.0.0
"#;

const PROMPT: &str = ">> ";

fn get_input() -> Result<String> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

fn main() -> Result<()> {
    println!("{SPLASH}");

    loop {
        print!("{PROMPT}");
        io::stdout().lock().flush()?;

        let input = get_input()?;
        let mut parser = Parser::new(Lexer::new(input));
        match parser.parse() {
            Ok(ast) => println!("{ast}"),
            Err(err) => eprintln!("{err}"),
        }
    }
}
