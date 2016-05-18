use rmp::value::{Value, Integer, Float};
use super::{FromValue, IntoValue};

pub trait ValueExt {
    // Map methods
    // - getters
    fn get<'a, K: IntoValue, V: FromValue<'a> = &'a Value>(&'a self, key: K) -> Option<V>;

    /// Lookup a value from a path, a dot-delimited string which indicate the keys to get
    /// throught the map to find the value.
    fn lookup<'a, V: FromValue<'a> = &'a Value>(&'a self, path: &str) -> Option<V>;

    // - setters
    fn set<K: IntoValue, V: IntoValue>(&mut self, key: K, value: V);
    fn remove<K: IntoValue>(&mut self, key: K) -> Option<Value>;

    // Convert methods
    fn from<T: IntoValue>(value: T) -> Self;
    fn to<'a, T: FromValue<'a>>(&'a self) -> Option<T>;

    // Test methods
    fn is_nil(&self) -> bool;
}

impl ValueExt for Value {
    // Map methods
    fn get<'a, K: IntoValue, V: FromValue<'a>>(&'a self, key: K) -> Option<V> {
        let map = match *self {
            Value::Map(ref entries) => entries,
            _ => return None,
        };

        let key = key.into_value();
        map.iter().find(|entry| entry.0 == key).and_then(|entry| entry.1.to())
    }

    fn lookup<'a, V: FromValue<'a>>(&'a self, path: &str) -> Option<V> {
        let mut cur = self;
        for key in path.split('.') {
            cur = match cur.get(key) {
                Some(value) => value,
                None => return None,
            };
        }
        cur.to()
    }

    fn set<K: IntoValue, V: IntoValue>(&mut self, key: K, value: V) {
        let mut map = match *self {
            Value::Map(ref mut entries) => entries,
            _ => return,
        };

        let key = key.into_value();
        let value = value.into_value();

        // FIXME: Workaround until non-lexical borrows are here.
        if let Some(index) = map.iter().enumerate()
                                .find(|&(_, ref entry)| entry.0 == key)
                                .map(|(index, _)| index)
        {
            map[index].1 = value;
        } else {
            map.push((key, value));
        }
    }

    fn remove<K: IntoValue>(&mut self, key: K) -> Option<Value> {
        let mut map = match *self {
            Value::Map(ref mut entries) => entries,
            _ => return None,
        };

        let key = key.into_value();
        let index = map.iter().enumerate()
                       .find(|&(_, ref entry)| entry.0 == key)
                       .map(|(index, _)| index);

        index.map(|index| map.swap_remove(index).1)
    }

    // Convert methods
    fn from<T: IntoValue>(value: T) -> Self {
        value.into_value()
    }

    fn to<'a, T: FromValue<'a>>(&'a self) -> Option<T> {
        T::from_value(self)
    }

    // Test methods
    fn is_nil(&self) -> bool {
        match *self {
            Value::Nil => true,
            _ => false,
        }
    }
}
