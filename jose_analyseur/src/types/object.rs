use super::JoseType;
use std::{borrow::Cow, collections::HashMap};

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub struct Object<'a, 'b> {
    donnee: HashMap<Cow<'a, str>, JoseType<'b, 'b>>,
}
