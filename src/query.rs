use std::{collections::HashMap, fmt::Debug};

pub trait ValueLike: erased_serde::Serialize + Debug + Send {}
impl<T> ValueLike for T where T: erased_serde::Serialize + Debug + Send {}

type Binding<'a> = Box<dyn ValueLike + 'a>;

/// A query building primitive that simplifies building the query string with binding parameters
#[derive(Debug, Default)]
pub struct Query<'binding> {
    bindings: HashMap<String, Binding<'binding>>,
    sql: String,
}

impl<'binding> Query<'binding> {
    /// Adds a raw substring to the query string
    pub fn add_raw(&mut self, raw: impl AsRef<str>) {
        self.sql.push_str(raw.as_ref());
    }

    /// Adds a raw substring ensuring there are spaces between the last substring and the next
    pub fn add_raw_ensuring_spaces(&mut self, raw: impl AsRef<str>) {
        let last_char_is_space = self
            .sql
            .chars()
            .last()
            .map(|c| c.is_whitespace())
            .unwrap_or_default();

        if !last_char_is_space && !self.sql.is_empty() {
            self.add_raw(" ");
        }

        self.add_raw(raw);
        self.add_raw(" ");
    }

    /// Adds a parameterized value to the query
    pub fn add_parameter<V>(&mut self, name: impl Into<String>, value: V)
    where
        V: ValueLike + 'binding,
    {
        let name = name.into();
        let param = format!("${}", name);

        self.add_raw(param);
        self.bindings.insert(name, Box::new(value));
    }

    /// Adds a parameterized value to the query ensuring there are spaces between the last substring and the next
    pub fn add_parameter_ensuring_spaces<V>(&mut self, name: impl Into<String>, value: V)
    where
        V: ValueLike + 'binding,
    {
        let name = name.into();
        let param = format!("${}", name);

        self.add_raw_ensuring_spaces(param);
        self.bindings.insert(name, Box::new(value));
    }

    /// Splits the query into a tuple, consuming it in the process
    pub fn split(self) -> (String, HashMap<String, Binding<'binding>>) {
        (self.sql, self.bindings)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut query = Query::default();

        let name = "John Smith";
        let age = 32;

        query.add_raw_ensuring_spaces("CREATE");
        query.add_raw("person:john");
        query.add_raw_ensuring_spaces("SET");

        query.add_raw("name");
        query.add_raw_ensuring_spaces("=");
        query.add_parameter("name", name);

        query.add_raw(", age");
        query.add_raw_ensuring_spaces("=");
        query.add_parameter("age", age);
        query.add_raw(";");

        let (sql, _) = query.split();
        assert_eq!(&sql, "CREATE person:john SET name = $name, age = $age;");
    }
}
