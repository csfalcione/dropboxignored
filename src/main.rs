use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod ignore;

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
    }
    Ok(())
}
