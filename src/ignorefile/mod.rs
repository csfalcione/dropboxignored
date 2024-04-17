mod parse;

use std::path::PathBuf;

use parse::MatchRule;
use regex::Regex;

pub fn build_matcher(base_dir: &PathBuf, line: &str) -> Result<impl Fn(&PathBuf) -> bool, String> {
    let match_line = MatchLine::new(base_dir, line)?;
    Ok(move |candidate: &PathBuf| match_line.matches(candidate))
}

struct MatchLine {
    rules: Vec<MatchRule>,
    pattern: Regex,
}

impl MatchLine {
    pub fn new(base_dir: &PathBuf, line: &str) -> Result<MatchLine, String> {
        let rules = parse::parse_line(line).map_err(|e| format!("{e}"))?.1;

        let pattern = MatchLine::create_regex(base_dir, &rules)?;

        Ok(MatchLine { rules, pattern })
    }

    pub fn matches(&self, candidate: &PathBuf) -> bool {
        if self.dir_only() && !candidate.is_dir() {
            return false;
        }
        let candidate: &str = match candidate.to_str() {
            Some(string) => string,
            None => {
                return false;
            }
        };
        self.pattern.is_match(candidate)
    }

    fn create_regex(base_dir: &PathBuf, rules: &[MatchRule]) -> Result<Regex, String> {
        let prefix = base_dir
            .to_str()
            .ok_or("base_dir isn't convertible to a &str")?;

        let platform_separator = if std::path::MAIN_SEPARATOR == '\\' {
            r"\\"
        } else {
            "/"
        };

        let optional_platform_separator = format!("{platform_separator}?");

        let suffix: String = rules
            .iter()
            .enumerate()
            .map(|(idx, rule)| -> &str {
                match rule {
                    MatchRule::Comment => "",
                    MatchRule::Text(text) => text,
                    MatchRule::QuestionMark => "?",
                    MatchRule::DoubleStar => r"[a-zA-Z0-9._\-\[\]/]*",
                    MatchRule::SingleStar => r"[a-zA-Z0-9._\-\[\]]*",
                    MatchRule::Separator => {
                        if idx < rules.len() - 1 {
                            platform_separator
                        } else {
                            // Incoming folder names often don't have trailing separators.
                            &optional_platform_separator
                        }
                    }
                }
            })
            .collect();

        let relative_modifier = if MatchLine::is_relative(rules) {
            r"[a-zA-Z0-9._\-/]*"
        } else {
            ""
        };

        let regex = format!("{}{}{}$", prefix, relative_modifier, suffix);
        println!("{:?} -> {}", rules, regex);

        Regex::new(&regex).map_err(|e| format!("{e}"))
    }

    fn is_relative(rules: &[MatchRule]) -> bool {
        rules
            .iter()
            .enumerate()
            .any(|(idx, rule)| *rule == MatchRule::Separator && idx != rules.len() - 1)
    }

    fn dir_only(&self) -> bool {
        matches!(
            self.rules
                .last()
                .map(|last_rule| *last_rule == MatchRule::Separator),
            Some(true)
        )
    }
}
