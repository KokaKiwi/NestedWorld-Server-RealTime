use rmp::value::{Value, Integer, Float};
use super::IntoValue;

pub trait ValueExt {
    // Constructor methods
    fn from<T: IntoValue>(value: T) -> Self;

    // Map methods
    // - getters
    fn get<K: IntoValue>(&self, key: K) -> Option<&Value>;

    /// Lookup a value from a path, a dot-delimited string which indicate the keys to get
    /// throught the map to find the value.
    fn lookup(&self, path: &str) -> Option<&Value>;

    fn get_bool<K: IntoValue>(&self, key: K) -> Option<bool> {
        self.get(key).and_then(ValueExt::as_bool)
    }
    fn get_int<K: IntoValue>(&self, key: K) -> Option<Integer> {
        self.get(key).and_then(ValueExt::as_int)
    }
    fn get_float<K: IntoValue>(&self, key: K) -> Option<Float> {
        self.get(key).and_then(ValueExt::as_float)
    }
    fn get_str<K: IntoValue>(&self, key: K) -> Option<&str> {
        self.get(key).and_then(ValueExt::as_str)
    }
    fn get_bytes<K: IntoValue>(&self, key: K) -> Option<&[u8]> {
        self.get(key).and_then(ValueExt::as_bytes)
    }
    fn get_array<K: IntoValue>(&self, key: K) -> Option<&[Value]> {
        self.get(key).and_then(ValueExt::as_array)
    }

    // - setters
    fn set<K: IntoValue, V: IntoValue>(&mut self, key: K, value: V);

    // Convert methods
    fn as_bool(&self) -> Option<bool>;
    fn as_int(&self) -> Option<Integer>;
    fn as_float(&self) -> Option<Float>;
    fn as_str(&self) -> Option<&str>;
    fn as_bytes(&self) -> Option<&[u8]>;
    fn as_array(&self) -> Option<&[Value]>;

    // Test methods
    fn is_nil(&self) -> bool;
}

impl ValueExt for Value {
    // Constructor methods
    fn from<T: IntoValue>(value: T) -> Self {
        value.into_value()
    }

    // Map methods
    fn get<K: IntoValue>(&self, key: K) -> Option<&Value> {
        let map = match *self {
            Value::Map(ref entries) => entries,
            _ => return None,
        };

        let key = key.into_value();
        map.iter().find(|entry| entry.0 == key).map(|entry| &entry.1)
    }

    fn lookup(&self, path: &str) -> Option<&Value> {
        let mut cur = self;
        for key in path.split('.') {
            cur = match cur.get(key) {
                Some(value) => value,
                None => return None,
            };
        }
        Some(cur)
    }

    fn set<K: IntoValue, V: IntoValue>(&mut self, key: K, value: V) {
        let mut map = match *self {
            Value::Map(ref mut entries) => entries,
            _ => return,
        };

        let key = key.into_value();
        let value = value.into_value();

        // FIXME: Workaround until non-lexical borrows are here.
        if let Some(index) = map.iter_mut().enumerate()
                                .find(|&(_, ref entry)| entry.0 == key)
                                .map(|(index, _)| index)
        {
            map[index].1 = value;
        } else {
            map.push((key, value));
        }
    }

    // Convert methods
    fn as_bool(&self) -> Option<bool> {
        match *self {
            Value::Boolean(value) => Some(value),
            _ => None,
        }
    }
    fn as_int(&self) -> Option<Integer> {
        match *self {
            Value::Integer(value) => Some(value),
            _ => None,
        }
    }
    fn as_float(&self) -> Option<Float> {
        match *self {
            Value::Float(value) => Some(value),
            _ => None,
        }
    }
    fn as_str(&self) -> Option<&str> {
        match *self {
            Value::String(ref value) => Some(&value),
            _ => None,
        }
    }
    fn as_bytes(&self) -> Option<&[u8]> {
        match *self {
            Value::Binary(ref value) => Some(&value),
            _ => None,
        }
    }
    fn as_array(&self) -> Option<&[Value]> {
        match *self {
            Value::Array(ref value) => Some(&value),
            _ => None,
        }
    }

    // Test methods
    fn is_nil(&self) -> bool {
        match *self {
            Value::Nil => true,
            _ => false,
        }
    }
}
