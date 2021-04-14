use super::JoseType;
use std::{borrow::Cow, collections::HashMap};

mod gender;
pub use gender::*;
mod key_value;
use crate::types::{parse_spaces_and_newlines, IResult, ParseValue};
pub use key_value::*;
use nom::bytes::complete::{tag};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::branch::alt;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Object<'a, 'b> {
    gender: Gender,
    inner: HashMap<Cow<'a, str>, JoseType<'b, 'b>>,
}

impl<'a, 'b> From<(Gender, HashMap<Cow<'a, str>, JoseType<'b, 'b>>)> for Object<'a, 'b> {
    fn from(from: (Gender, HashMap<Cow<'a, str>, JoseType<'b, 'b>>)) -> Self {
        Object {
            gender: from.0,
            inner: from.1,
        }
    }
}

const OBJECT_DELIM_BEGIN: &str = "OBJET ";
const OBJECT_DELIM_END: &str = "TEJBO";
const LINE_SEPARATOR: &str = " ;";
const OBJECT_LAST_LINE: &str = ".";

impl<'a, 'b> ParseValue<'a, 'b> for Object<'a, 'b> {
    type Input = &'a str;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>> {
        context(
            "nom parsing object",
            delimited(
                tag(OBJECT_DELIM_BEGIN),
                tuple((
                    gender::Gender::parse,
                    parse_spaces_and_newlines,
                    many0(tuple((
                        key_value::KeyValue::parse,
                        alt((tag(LINE_SEPARATOR), tag(OBJECT_LAST_LINE))),
                        parse_spaces_and_newlines,
                    ))),
                )),
                tag(OBJECT_DELIM_END),
            ),
        )(input)
        .map(|(next_input, res)| {
            dbg!(res.clone());
            (
                next_input,
                JoseType::Object(Object {
                    gender: res.0,
                    inner: HashMap::new(),
                }),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Object, Gender};
    use crate::types::{JoseType, ParseValue};
    use std::collections::HashMap;

    #[test]
    fn test_parse_kv() {
        assert_eq!(
            Object::parse("OBJET Féminin TEJBO").unwrap(),
            ("", JoseType::Object((Gender::Feminine, HashMap::new()).into())),
        );
    }

    #[test]
    fn test_parse_object_error() {
        assert!(Object::parse("NOTOBJECT").err().is_some());
    }
}
