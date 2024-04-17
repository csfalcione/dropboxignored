# dropboxignored

## Usage

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

#### File watcher
```
dropboxignored watch [OPTIONS] <PATH>

Options:
  -f <IGNORE_FILE>
```
Where PATH is your dropbox directory (or a subdir). `IGNORE_FILE` defaults to a `.dropboxignore` file in the `PATH` directory if the `-f` switch isn't specified.
