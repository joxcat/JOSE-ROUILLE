use crate::types::{ParseValue, IResult, JoseType};
use nom::error::{context, VerboseError};
use nom::bytes::complete::{tag, take};
use nom::sequence::{preceded};
use crate::errors::JoseError;

#[derive(Debug, Eq, PartialEq)]
pub struct Escape {
    inner: char,
}

impl From<&str> for Escape {
    fn from(from: &str) -> Self {
        dbg!(&from);
        Self { inner: from.chars().next().unwrap() }
    }
}

const ESCAPE: &str = "ÉCHAPPER ";

impl<'a,'b> ParseValue<'a,'b> for Escape {
    type Input = &'a str;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>> {
        context(
            "nom parsing escaped value",
            preceded(tag(ESCAPE), take(1_usize)),
        )(input)
            .map_err(|_: nom::Err<VerboseError<&str>>| JoseError::NotEscaped.into())
            .map(|(next_input, res)| (
                next_input,
                JoseType::Escape(Self::from(res)),
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::Escape;
    use crate::errors::JoseError;
    use crate::types::{JoseType, ParseValue};

    #[test]
    fn test_parse_escape() {
        assert_eq!(
            Escape::parse("ÉCHAPPER n").unwrap(),
            ("", JoseType::Escape(Escape::from("n")))
        );
    }

    #[test]
    fn test_parse_escape_error() {
        assert_eq!(
            Escape::parse("NOTESCAPE u").err().unwrap().to_string(),
            Some(nom::Err::from(JoseError::NotEscaped))
                .unwrap()
                .to_string(),
        );
    }
}
