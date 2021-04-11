use std::borrow::Cow;
use crate::types::{ParseValue, IResult, JoseType};
use nom::error::{context, VerboseError};
use crate::errors::JoseError;
use nom::sequence::delimited;
use nom::bytes::complete::{tag, take_until};

#[derive(Debug, Eq, PartialEq)]
pub struct String<'a> {
    inner: Cow<'a, str>,
}

impl<'a> From<&'a str> for String<'a> {
    fn from(from: &'a str) -> Self {
        Self { inner: Cow::from(from) }
    }
}

const STRING_DELIM_BEGIN: &str = "«";
const STRING_DELIM_END: &str = "»";


impl<'a, 'b> ParseValue<'a, 'b> for String<'b> {
    type Input = &'a str;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>> {
        context(
            "nom parsing string value",
            delimited(tag(STRING_DELIM_BEGIN), take_until(STRING_DELIM_END), tag(STRING_DELIM_END)),
        )(input)
            .map_err(|_: nom::Err<VerboseError<&str>>| JoseError::NotAString.into())
            .map(|(next_input, res)| (
                next_input,
                JoseType::String(String::from(res)),
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::String;
    use crate::errors::JoseError;
    use crate::types::{JoseType, ParseValue};

    #[test]
    fn test_parse_string() {
        assert_eq!(
            String::parse("« Seine-Maritime »").unwrap(),
            ("", JoseType::String(String::from(" Seine-Maritime ")))
        );
        assert_eq!(
            String::parse("«Rhône»").unwrap(),
            ("", JoseType::String(String::from("Rhône")))
        );
    }

    #[test]
    fn test_parse_string_error() {
        assert_eq!(
            String::parse("NOTSTRING").err().unwrap().to_string(),
            Some(nom::Err::from(JoseError::NotAString))
                .unwrap()
                .to_string(),
        );
    }
}
