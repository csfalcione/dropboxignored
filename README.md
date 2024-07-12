<p align="center"><img alt-text="Image depicting the mass of celestial bodies, where node_modules is heavier than a black hole" src="https://preview.redd.it/tfugj4n3l6ez.png?auto=webp&s=b8163176d8482d5e78ac631e16b7973a52e3b188" height=355 /></p>

# dropboxignored
Dropbox is a useful development tool to sync in-progress work across computers independently and in addition to traditional source control. An emergent problem is irrelevant files/folders syncing, wasting bandwidth and Dropbox storage. Such undesirable folders usually include project build artifacts and editor metadata. Dropbox provides the capability to use a filesystem-level attribute to ignore directories and files, but can only be applied retroactively after creation. Ideally, then, one would be able to define rules for automatically ignoring directories and files to prevent the race where Dropbox starts syncing the thing that should be ignored before the attribute can be applied.

Enter `dropboxignored`. Define a `.dropboxignore` file with the same syntax and (mostly) semantics as a `.gitignore` in the root of your Dropbox folder, start `dropboxignored`, and create new projects without worry of polluting your Dropbox.

## Installation
```
cargo install dropboxignored
```

## Usage

#### File watcher
```
dropboxignored watch [OPTIONS] <PATH>

Options:
  -f <IGNORE_FILE>
```
Where PATH is your dropbox directory (or a subdir). `IGNORE_FILE` defaults to a `.dropboxignore` file in the `PATH` directory if the `-f` switch isn't specified.

#### Overall
```
Usage: dropboxignored <COMMAND>

Commands:
  ignore    Ignore given file [aliases: i]
  unignore  Unignore given file [aliases: u]
  check     Check whether the given file is ignored [aliases: c]
  watch     Watch a given directory, ignoring and unignoring files as they're created and renamed [aliases: w]
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

#### Explicitly checking and marking files
```
dropboxignored check <PATH>
dropboxignored ignore <PATH>
dropboxignored unignore <PATH>
```

