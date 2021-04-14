use crate::errors::JoseError;
use crate::types::{parse_spaces_and_newlines, string, IResult, JoseType, ParseValue};
use nom::bytes::complete::{tag};
use nom::error::context;
use nom::sequence::{preceded, tuple};
use std::borrow::Cow;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KeyValue<'a, 'b> {
    key: Cow<'a, str>,
    value: JoseType<'a, 'b>,
}

impl<'a, 'b> From<(&'a str, JoseType<'a, 'b>)> for KeyValue<'a, 'b> {
    fn from(from: (&'a str, JoseType<'a, 'b>)) -> Self {
        KeyValue {
            key: Cow::from(from.0),
            value: from.1,
        }
    }
}

const KEY_PREFIX: &str = "— ";
const VALUE_PREFIX: &str = ": ";

impl<'a, 'b> KeyValue<'a, 'b> {
    pub(crate) fn parse(input: &'a str) -> IResult<&str, Self> {
        context(
            "nom parsing key_value",
            tuple((
                preceded(tag(KEY_PREFIX), string::String::parse),
                parse_spaces_and_newlines,
                preceded(tag(VALUE_PREFIX), JoseType::parse),
            )),
        )(input)
        .and_then(|(next_input, res)| match res.0 {
            JoseType::String(s) => Ok((
                next_input,
                KeyValue {
                    key: s.inner,
                    value: res.2,
                },
            )),
            _ => Err(JoseError::NotAKv.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::KeyValue;
    use crate::types::{bool, JoseType};

    #[test]
    fn test_parse_kv() {
        assert_eq!(
            KeyValue::parse("— « Écoles ouvertes » : Faux").unwrap(),
            (
                "",
                KeyValue::from((" Écoles ouvertes ", JoseType::Bool(bool::Bool::Faux)))
            )
        );
        assert_eq!(
            KeyValue::parse(
                "— « Écoles espacees » : \
            DÉBUT  FIN"
            )
            .unwrap(),
            (
                "",
                KeyValue::from((" Écoles espacees ", JoseType::Table(Vec::new().into())))
            )
        );
    }

    #[test]
    fn test_parse_kv_error() {
        assert!(KeyValue::parse("NOTKV").err().is_some());
    }
}
