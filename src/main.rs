use std::fs;
use std::io;
use std::path;
use std::process;

use anyhow::Context;
use itertools::Itertools;

mod dialoguer_fork;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    extract: String,
    invoke: String,
}

fn run_command(command: Vec<&str>, value: &str) -> anyhow::Result<()> {
    let mut proc = process::Command::new(command[0]);
    if command.len() > 1 {
        proc.args(&command[1..]);
    }
    proc.arg(value).status()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let config = path::Path::new(&home::home_dir().context("Could not locate home directory")?)
        .join(".config")
        .join("exvoker")
        .join("exvoker.toml");

    let contents =
        fs::read_to_string(config).context("Cannot find ~/.config/evoker/exvoker.toml file")?;
    let configuration: Config = toml::from_str(&contents)?;
    let invoke = configuration.invoke.split(' ').collect::<Vec<&str>>();
    if invoke.is_empty() {
        anyhow::bail!("Command is empty");
    }
    let extract = regex::Regex::new(&configuration.extract)?;

    let stdin = io::read_to_string(io::stdin())?;

    let matches = extract
        .find_iter(stdin.as_str())
        .map(|x| x.as_str())
        .collect::<Vec<_>>();
    let matches = matches.into_iter().unique().collect::<Vec<_>>();
    if !matches.is_empty() {
        ctrlc::set_handler(move || {
            let term = console::Term::stdout();
            let _ = term.show_cursor();
        })?;
        let theme = dialoguer_fork::ColorfulTheme::default();
        let interaction = dialoguer_fork::FuzzySelect::with_theme(&theme)
            .default(0)
            .items(&matches)
            .interact_opt();
        if let Ok(Some((is_enter, index))) = interaction {
            let value = matches[index];
            if is_enter {
                let input = dialoguer_fork::Input::<String>::with_theme(&theme)
                    .with_initial_text(value)
                    .interact_text();
                if let Ok(modified_value) = input {
                    run_command(invoke, modified_value.as_str())?;
                }
                // Else error, e.g. due to Control-C.
            } else {
                run_command(invoke, value)?;
            }
        }
        // Else error, e.g. due to Control-C.
    }

    Ok(())
}
