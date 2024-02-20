use std::{path::PathBuf, sync::mpsc::channel};

use notify::{event::ModifyKind, EventKind, RecursiveMode, Watcher};

use crate::ignore::{ignore_path, unignore_path};

#[derive(PartialEq, Clone, Copy)]
pub enum IgnoreDecision {
    None,
    Ignore,
    Unignore,
}

pub fn watch<F>(root: &PathBuf, mut decider: F) -> Result<(), notify::Error>
where
    F: FnMut(&PathBuf) -> IgnoreDecision + Send + 'static,
{
    let (tx, rx) = channel();

    let handle = |decision, path: &PathBuf| {
        let result = match decision {
            IgnoreDecision::None => {
                println!("not touching {}", path.display());
                Ok(())
            }
            IgnoreDecision::Ignore => {
                println!("ignoring {}", path.display());
                ignore_path(path)
            }
            IgnoreDecision::Unignore => {
                println!("unignoring {}", path.display());
                unignore_path(path)
            }
        };
        if let Err(e) = result {
            eprintln!("error handling {}: {:?}", path.display(), e);
            return;
        }
    };

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(root, RecursiveMode::Recursive)?;

    for result in rx {
        match result {
            Ok(event) => match event.kind {
                EventKind::Create(_) => {
                    for path in event.paths {
                        handle(decider(&path), &path);
                    }
                }
                EventKind::Modify(ModifyKind::Name(notify::event::RenameMode::Both)) => {
                    let from_path = &event.paths[0];
                    let to_path = &event.paths[1];
                    let from_decision = decider(from_path);
                    let to_decision = decider(to_path);
                    // Is this sufficient for Ignore -> None?
                    if from_decision == to_decision {
                        continue;
                    }
                    handle(to_decision, to_path);
                }
                _ => (),
            },
            Err(e) => eprintln!("error: {:?}", e),
        }
    }

    Ok(())
}
