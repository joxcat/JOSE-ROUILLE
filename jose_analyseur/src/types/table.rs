use super::JoseType;

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub struct Table<'a, 'b> {
    donnee: Vec<JoseType<'a, 'b>>,
}
