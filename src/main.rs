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
    println!("Password Vault");
    println!("Type 'help' to access more info");

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
    #[command(
        about = "Add Password",
        long_about = "Add Password: Service, Username, Password",
        visible_aliases = &["a","0"],
    )]
    Add,
    #[command(
        about = "Get Password",
        long_about = "Get Password: Fuzz Searching with Service Input",
        visible_aliases = &["g","1"],
    )]
    Get,
    #[command(
        about = "List Passwords", 
        long_about = "List All Passwords",
        visible_aliases = &["l","2"])]
    List,
    #[command(
        about = "Delete Password",
        long_about = "Delete Password: Exact Delete with Service Input",
        visible_aliases = &["d","3"]
    )]
    Delete,
    #[command(
        about = "Quit",
        long_about = "Quit Prog",
        visible_aliases = &["q","4"])]
    Quit,
}
