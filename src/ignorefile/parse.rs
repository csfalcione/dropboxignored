use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::is_alphanumeric;
use nom::combinator::all_consuming;
use nom::combinator::verify;
use nom::multi::many0;
use nom::multi::many1_count;
use nom::IResult;
use nom::Parser;

#[derive(Debug, Clone, PartialEq)]
pub enum MatchRule {
    /// Literal text.
    Text(String),
    /// Wildcard over Text matching 0 or more characters.
    SingleStar,
    /// Wildcard over Text and Separator matching 0 or more characters or separators.
    DoubleStar,
    /// Wildcard over Text matching 0 or 1 characters.
    QuestionMark,
    /// Literal '/' delimiting directories.
    Separator,
}

pub fn parse_line(line: &str) -> IResult<&str, Vec<MatchRule>> {
    let separator =
        verify(many1_count(tag("/")), |count| *count == 1).map(|_| MatchRule::Separator);
    let stars = many1_count(tag("*")).map(|count| {
        if count == 1 {
            return MatchRule::SingleStar;
        }
        MatchRule::DoubleStar
        // Any further stars after two are treated as single stars, which is semantically equivalent
        // to just two stars.
    });
    let question_mark = tag("?").map(|_| MatchRule::QuestionMark);
    let text = take_while1(|c| c == ('_') || is_alphanumeric(c as u8))
        .map(|text: &str| MatchRule::Text(text.to_string()));

    all_consuming(many0(alt((separator, stars, question_mark, text))))(line)
}

#[cfg(test)]
mod test {
    use super::parse_line;
    use super::MatchRule::*;

    #[test]
    fn test_node_modules() {
        assert_eq!(
            parse_line("/**/node_modules/"),
            Ok((
                "",
                vec![
                    Separator,
                    DoubleStar,
                    Separator,
                    Text("node_modules".to_string()),
                    Separator
                ]
            ))
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(parse_line(""), Ok(("", vec![])));
    }

    #[test]
    fn test_double_slash() {
        assert!(parse_line("//").is_err());
    }
}
