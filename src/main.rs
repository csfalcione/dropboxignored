use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    Watch { path: PathBuf },
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
        Commands::Watch { path } => {
            println!("attempting to watch {}", path.display());
            watcher::watch(path, |path| {
                if path.is_dir() {
                    if path.ends_with("node_modules") {
                        return IgnoreDecision::Ignore;
                    }
                    if path.iter().any(|part| part == "node_modules") {
                        return IgnoreDecision::None;
                    }
                    return IgnoreDecision::Unignore;
                }
                return IgnoreDecision::None;
            })
            .unwrap();
        }
    }
    Ok(())
}
