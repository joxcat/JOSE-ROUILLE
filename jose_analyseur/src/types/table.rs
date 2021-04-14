use super::JoseType;
use crate::types::{IResult, ParseValue};
use nom::bytes::complete::tag;
use nom::error::context;
use nom::multi::separated_list0;
use nom::sequence::delimited;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Table<'a, 'b> {
    inner: Vec<JoseType<'a, 'b>>,
}

impl<'a, 'b> From<Vec<JoseType<'a, 'b>>> for Table<'a, 'b> {
    fn from(from: Vec<JoseType<'a, 'b>>) -> Self {
        Self { inner: from }
    }
}

const TABLE_DELIM_BEGIN: &str = "DÉBUT ";
const TABLE_DELIM_END: &str = " FIN";
const SEPARATOR: &str = " ; ";

impl<'a, 'b> ParseValue<'a, 'b> for Table<'a, 'b> {
    type Input = &'a str;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>> {
        context(
            "nom parsing table",
            delimited(
                tag(TABLE_DELIM_BEGIN),
                separated_list0(tag(SEPARATOR), JoseType::parse),
                tag(TABLE_DELIM_END),
            ),
        )(input)
        .map(|(next_input, res)| (next_input, JoseType::Table(Table { inner: res })))
    }
}

#[cfg(test)]
mod tests {
    use super::Table;
    use crate::types::{string, JoseType, ParseValue};

    #[test]
    fn test_parse_table() {
        assert_eq!(
            Table::parse("DÉBUT « Seine-Maritime » ; « Eure » ; « Rhône » FIN").unwrap(),
            (
                "",
                JoseType::Table(Table::from(vec![
                    JoseType::String(string::String::from(" Seine-Maritime ")),
                    JoseType::String(string::String::from(" Eure ")),
                    JoseType::String(string::String::from(" Rhône ")),
                ]))
            )
        );
    }

    #[test]
    fn test_parse_table_error() {
        assert!(Table::parse("NOTTABLE").err().is_some());
    }
}
