use crate::{call_control_flow, errors, pentry::Info, save_to_file, Cli, Commands, FILE_PATH};
use clap::Parser;
use errors::*;
use rust_fuzzy_search::fuzzy_search;
use std::{collections::HashMap, io::Write};

// ControlFlow
#[derive(Debug)]
pub enum Control<T> {
    Next(T),
    Continue,
    Quit,
}

pub fn read_input(prompt: &str) -> Result<Control<String>> {
    write!(std::io::stdout(), "$ {prompt}")?;
    std::io::stdout().flush()?;
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer)?;
    let buffer = buffer.trim().to_string();
    if buffer.eq_ignore_ascii_case("continue")
        || buffer.eq_ignore_ascii_case("ct")
        || buffer.is_empty()
    {
        Ok(Control::Continue)
    } else {
        Ok(Control::Next(buffer))
    }
}

pub fn handle_command(line: String, state: &mut Vec<Info>) -> Result<Control<()>> {
    let args = Cli::try_parse_from(line.split(" ").into_iter())?;
    match args.command {
        Commands::Add => {
            let info = call_control_flow!(Info::info_read()?);
            state.push(info);
            println!("Done!");
            Ok(Control::Next(()))
        }
        Commands::Get => {
            let sv = call_control_flow!(read_input("Please input your service for searching: ")?)
                .to_lowercase();

            let info_hash: HashMap<String, &Info> = state
                .iter()
                .map(|info| (info.service.to_lowercase(), info))
                .collect();

            let services: Vec<&str> = info_hash.keys().map(String::as_str).collect();

            let mut results = fuzzy_search(&sv, &services);
            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            let mut found = false;

            results.iter().take(10).for_each(|(sv, score)| {
                if let Some(info) = info_hash.get(*sv) {
                    if *score >= 0.5 {
                        println!("Info: {:?}, Relation Score: {}", info, score);
                        found = true;
                    }
                }
            });

            if !found {
                println!("Not Found!");
            } else {
                println!("Done!");
            }

            Ok(Control::Next(()))
        }

        Commands::List => {
            state.iter().for_each(|info| println!("Info: {:?}", info));
            println!("Done!");
            Ok(Control::Next(()))
        }

        Commands::Delete => {
            let sv =
                call_control_flow!(read_input("Please input your service for exact delete: ")?);
            state.retain(|info| info.service != sv);
            println!("Done!");
            Ok(Control::Next(()))
        }

        Commands::Quit => {
            save_to_file(&state, FILE_PATH)?;
            Ok(Control::Quit)
        }
    }
}
