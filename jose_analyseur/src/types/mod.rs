use crate::errors::JoseError;
use nom::error::VerboseError;

mod bool;
mod escape;
mod null;
mod object;
mod string;
mod table;
mod integer;

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum JoseType<'a, 'b> {
    Table(table::Table<'a, 'b>),
    Bool(bool::Bool),
    String(string::String<'a>),
    Null(null::Null<'a>),
    Integer(integer::Integer<'a>),
    Escape(escape::Escape),
    Object(object::Object<'a, 'b>),
}

pub type IResult<T, U> = nom::IResult<T, U, VerboseError<JoseError>>;

pub trait ParseValue<'a, 'b> {
    type Input;
    fn parse(input: Self::Input) -> IResult<Self::Input, JoseType<'a, 'b>>;
}
