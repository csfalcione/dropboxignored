use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};

use crate::watcher::IgnoreDecision;

mod ignore;
mod ignorefile;
mod watcher;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ignore given file.
    #[command(visible_alias("i"))]
    Ignore { path: PathBuf },
    /// Unignore given file.
    #[command(visible_alias("u"))]
    Unignore { path: PathBuf },
    /// Check whether the given file is ignored.
    #[command(visible_alias("c"))]
    Check { path: PathBuf },
    /// Watch a given directory, ignoring and unignoring files as they're created and renamed.
    #[command(visible_alias("w"))]
    Watch {
        path: PathBuf,
        #[arg(short('f'))]
        ignore_file: Option<PathBuf>,
    },
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ignore { path } => {
            ignore::ignore_path(path)?;
            println!("ignored {}", path.display());
        }
        Commands::Unignore { path } => {
            ignore::unignore_path(path)?;
            println!("unignored {}", path.display());
        }
        Commands::Check { path } => {
            let is_ignored = ignore::check_path(path)?;
            if is_ignored {
                println!("{} is ignored", path.display());
            } else {
                println!("{} is not ignored", path.display());
            }
        }
        Commands::Watch { path, ignore_file } => {
            let path = &path.to_owned().canonicalize()?;
            println!("Attempting to watch {}", path.display());
            let ignore_file_path =
                ignore_file
                    .as_ref()
                    .map(|x| x.to_owned())
                    .unwrap_or_else(|| {
                        let mut path = path.clone();
                        path.push(".dropboxignore");
                        path
                    });

            let contents: Vec<String> = fs::read_to_string(ignore_file_path).map(|contents| {
                contents
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| line.len() > 0)
                    .map(|x| x.to_owned())
                    .collect()
            })?;

            if contents.len() == 0 {
                eprintln!("Empty or nonexistent .dropboxignore");
                return Ok(());
            }

            let (matchers, errors): (Vec<Result<_, _>>, Vec<Result<_, _>>) = contents
                .iter()
                .map(|line| ignorefile::build_matcher(path, line))
                .enumerate()
                .map(|(idx, result)| result.map_err(|err| format!("line {}: {}", idx + 1, err)))
                .partition(Result::is_ok);

            errors
                .into_iter()
                .map(|err_result| err_result.err().unwrap())
                .for_each(|err_msg| eprintln!("[E] {err_msg}"));

            let matchers: Vec<_> = matchers.into_iter().map(Result::unwrap).collect();

            watcher::watch(path, move |candidate| {
                if matchers.iter().any(|matcher| matcher(candidate)) {
                    return IgnoreDecision::Ignore;
                }
                return IgnoreDecision::None;
            })
            .unwrap();
        }
    }
    Ok(())
}
