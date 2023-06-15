use crate::value::Ident;

#[derive(Debug)]
pub struct Select<'a> {
    from: Option<Ident<'a>>,
}

impl<'a> Select<'a> {
    pub fn from(mut self, val: impl Into<Ident<'a>>) -> Select<'a> {
        self.from = Some(val.into());
        self
    }
}
