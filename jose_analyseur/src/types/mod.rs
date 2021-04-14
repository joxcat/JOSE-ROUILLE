use crate::errors::VerboseJoseError;
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::error::context;

mod bool;
mod escape;
mod integer;
mod null;
mod object;
mod string;
mod table;

#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum JoseType<'a, 'b> {
    Table(table::Table<'a, 'b>),
    Bool(bool::Bool),
    String(string::String<'a>),
    Null(null::Null<'a>),
    Integer(integer::Integer<'a>),
    Escape(escape::Escape),
    Object(object::Object<'a, 'b>),
}

pub type IResult<T, U> = nom::IResult<T, U, VerboseJoseError>;

#[allow(clippy::result_unit_err)]
pub fn parse_spaces_and_newlines(input: &str) -> IResult<&str, ()> {
    context(
        "nom parsing whitespaces",
        take_while(|c: char| c.is_whitespace() || c == '\n'),
    )(input)
    .map(|(next_input, _)| (next_input, ()))
}

pub trait ParseValue<'a, 'b> {
    type Input;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>>;
}

impl<'a, 'b> ParseValue<'a, 'b> for JoseType<'a, 'b> {
    type Input = &'a str;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>> {
        context(
            "nom parsing jose",
            alt((
                bool::Bool::parse,
                null::Null::parse,
                escape::Escape::parse,
                string::String::parse,
                table::Table::parse,
            )),
        )(input)
    }
}
