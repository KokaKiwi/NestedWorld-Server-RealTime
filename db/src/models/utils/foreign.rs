use postgres::Connection;
use super::Model;

#[derive(Debug, Clone)]
pub struct Relation<T: Model> {
    id: i32,
    value: Option<T>,
}

impl<T: Model> Relation<T> {
    pub fn new(id: i32) -> Relation<T> {
        Relation {
            id: id,
            value: None,
        }
    }

    pub fn with_value(id: i32, value: T) -> Relation<T> {
        Relation {
            id: id,
            value: Some(value),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn set(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn fetch(&mut self, conn: &Connection) -> ::postgres::Result<Option<&T>> {
        self.value = try!(T::get_by_id(conn, self.id));
        Ok(self.get())
    }

    pub fn get_or_fetch(&mut self, conn: &Connection) -> ::postgres::Result<Option<&T>> {
        match self.value {
            Some(ref value) => Ok(Some(value)),
            None => self.fetch(conn),
        }
    }
}
