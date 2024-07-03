use std::io::Write;

use clap::{Parser, Subcommand};
use errors::*;
use pentry::*;
use untils::*;

mod errors;
mod macros;
mod pentry;
mod untils;

const ART: &str = r#"
    ____             __     _    __            ____ 
   / __ \__  _______/ /_   | |  / /___ ___  __/ / /_
  / /_/ / / / / ___/ __/   | | / / __ `/ / / / / __/
 / _, _/ /_/ (__  ) /_     | |/ / /_/ / /_/ / / /_  
/_/ |_|\__,_/____/\__/     |___/\__,_/\__,_/_/\__/  

"#;

const FILE_PATH: &str = "vaults.json";

fn main() -> Result<()> {
    println!("{}", ART);

    let mut state = load_from_file(FILE_PATH)?;

    loop {
        let line = read_input("")?;
        match line {
            Control::Continue => continue,
            Control::Quit => break,
            Control::Next(line) => {
                match handle_command(line, &mut state) {
                    Ok(line) => match line {
                        Control::Quit => break,
                        _ => continue,
                    },
                    Err(e) => {
                        write!(std::io::stdout(), "{e}")?;
                        std::io::stdout().flush()?
                    }
                };
            }
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(name = "add", alias = "a", alias = "0")]
    Add,
    #[clap(name = "get", alias = "g", alias = "1")]
    Get,
    #[clap(name = "List", alias = "ls", alias = "2")]
    List,
    #[clap(name = "Delete", alias = "d", alias = "3")]
    Delete,
    #[clap(name = "Quit", alias = "q", alias = "4")]
    Quit,
}
