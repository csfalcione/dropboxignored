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
        let rules = MatchLine::transform_parsed_rules(rules);

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

    fn transform_parsed_rules(mut rules: Vec<MatchRule>) -> Vec<MatchRule> {
        if MatchLine::is_relative(&rules) {
            // Prepend a separator and doublestar to turn the relative rule into an absolute rule.
            rules
                .splice(0..0, [MatchRule::Separator, MatchRule::DoubleStar])
                .for_each(drop);
        }

        rules
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

        let regex = format!("{prefix}{relative_modifier}{suffix}$");
        println!("{:?} -> {}", rules, regex);

        Regex::new(&regex).map_err(|e| format!("{e}"))
    }

    fn is_relative(rules: &[MatchRule]) -> bool {
        rules
            .iter()
            .enumerate()
            // `rule` being a Separator implies that we're on the last index.
            .all(|(idx, rule)| *rule != MatchRule::Separator || idx == rules.len() - 1)
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

#[cfg(test)]
mod tests {
    use super::*;

    mod build_matcher {
        use super::*;

        #[test]
        fn test_absolute() -> Result<(), String> {
            let matcher = build_matcher(&PathBuf::from("/base"), "/**/node_modules")?;

            assert!(matcher(&PathBuf::from(
                "/base/code/js/project/node_modules"
            )));

            Ok(())
        }

        #[test]
        fn test_relative() -> Result<(), String> {
            let matcher = build_matcher(&PathBuf::from("/base"), "node_modules")?;

            assert!(matcher(&PathBuf::from(
                "/base/code/js/project/node_modules"
            )));

            Ok(())
        }
    }

    mod is_relative {
        use super::*;

        fn text(string: &str) -> MatchRule {
            MatchRule::Text(string.to_string())
        }

        #[test]
        fn test_name_only() {
            assert!(MatchLine::is_relative(&[text("path")]))
        }

        #[test]
        fn test_trailing_separator() {
            assert!(MatchLine::is_relative(&[
                text("path"),
                MatchRule::Separator
            ]))
        }

        #[test]
        fn test_absolute() {
            assert!(!MatchLine::is_relative(&[
                MatchRule::Separator,
                text("path")
            ]))
        }
    }
}
