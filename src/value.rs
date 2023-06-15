use std::fmt::Debug;
use surrealdb::sql::Thing;

/// A value inserted into a query
#[derive(Debug)]
pub enum Value<'a> {
    Raw(&'a str),
    Param(BoxedValue<'a>),
}

/// An identifier. Can be either a table, or a record id ([Thing])
#[derive(Debug, Clone)]
pub enum Ident<'a> {
    Table(&'a str),
    Record(Thing),
}

impl From<Thing> for Ident<'_> {
    fn from(value: Thing) -> Self {
        Ident::Record(value)
    }
}

impl<'a> From<&'a str> for Ident<'a> {
    fn from(value: &'a str) -> Self {
        Ident::Table(value)
    }
}

pub type BoxedValue<'a> = Box<dyn ValueLike + 'a>;
pub trait ValueLike: erased_serde::Serialize + Debug + Send {}
impl<T> ValueLike for T where T: erased_serde::Serialize + Debug + Send {}
