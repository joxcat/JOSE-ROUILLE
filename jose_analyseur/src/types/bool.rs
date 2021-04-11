use crate::errors::JoseError;
use crate::types::{IResult, JoseType, ParseValue};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::error::{context, VerboseError};

#[derive(Debug, Eq, PartialEq)]
pub enum Bool {
    Vrai,
    Faux,
}

impl<'a, 'b> ParseValue<'a, 'b> for Bool {
    type Input = &'a str;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>> {
        context("nom parsing boolean", alt((tag("Vrai"), tag("Faux"))))(input)
            .map_err(|_: nom::Err<VerboseError<&str>>| JoseError::NotABool.into())
            .and_then(|(next_input, res)| match res {
                "Vrai" => Ok((next_input, JoseType::Bool(Self::Vrai))),
                "Faux" => Ok((next_input, JoseType::Bool(Self::Faux))),
                _ => Err(JoseError::NotABool.into()),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Bool;
    use crate::errors::JoseError;
    use crate::types::{JoseType, ParseValue};

    #[test]
    fn test_parse_bool() {
        assert_eq!(
            Bool::parse("Vrai").unwrap(),
            ("", JoseType::Bool(Bool::Vrai))
        );
        assert_eq!(
            Bool::parse("Faux").unwrap(),
            ("", JoseType::Bool(Bool::Faux))
        );
    }

    #[test]
    fn test_parse_bool_error() {
        assert_eq!(
            Bool::parse("NOTBOOL").err().unwrap().to_string(),
            Some(nom::Err::from(JoseError::NotABool))
                .unwrap()
                .to_string(),
        );
    }
}
