mod parse;

use parse::MatchRule;

pub struct MatchLine {
    dir_only: bool,
    rules: Vec<MatchRule>,
}
