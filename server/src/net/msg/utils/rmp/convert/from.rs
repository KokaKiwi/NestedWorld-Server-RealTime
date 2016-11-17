use chrono::{Date, DateTime, UTC};
use rmp::value::Value;
use std::collections::HashMap;

pub trait FromValue<'a>: Sized {
    fn from_value(value: &'a Value) -> Option<Self>;
}

impl<'a> FromValue<'a> for bool {
    fn from_value(value: &'a Value) -> Option<bool> {
        match *value {
            Value::Boolean(value) => Some(value),
            _ => None,
        }
    }
}

macro_rules! int_convert {
    ($ty:ty) => {
        impl<'a> FromValue<'a> for $ty {
            fn from_value(value: &'a Value) -> Option<$ty> {
                use rmp::value::Integer;

                match *value {
                    Value::Integer(Integer::U64(value)) => Some(value as $ty),
                    Value::Integer(Integer::I64(value)) => Some(value as $ty),
                    _ => None,
                }
            }
        }
    };
}

int_convert!(i8);
int_convert!(i16);
int_convert!(i32);
int_convert!(i64);
int_convert!(u8);
int_convert!(u16);
int_convert!(u32);
int_convert!(u64);

macro_rules! float_convert {
    ($ty:ty) => {
        impl<'a> FromValue<'a> for $ty {
            fn from_value(value: &'a Value) -> Option<Self> {
                use rmp::value::Float;

                match *value {
                    Value::Float(Float::F32(value)) => Some(value as _),
                    Value::Float(Float::F64(value)) => Some(value as _),
                    _ => None,
                }
            }
        }
    };
}

float_convert!(f32);
float_convert!(f64);

impl<'a> FromValue<'a> for &'a str {
    fn from_value(value: &'a Value) -> Option<&'a str> {
        match *value {
            Value::String(ref value) => Some(value),
            _ => None,
        }
    }
}

impl<'a> FromValue<'a> for String {
    fn from_value(value: &'a Value) -> Option<String> {
        match *value {
            Value::String(ref value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl<'a> FromValue<'a> for &'a [u8] {
    fn from_value(value: &'a Value) -> Option<&'a [u8]> {
        match *value {
            Value::Binary(ref data) => Some(data),
            _ => None,
        }
    }
}

impl<'a> FromValue<'a> for Vec<u8> {
    fn from_value(value: &'a Value) -> Option<Vec<u8>> {
        match *value {
            Value::Binary(ref data) => Some(data.clone()),
            _ => None,
        }
    }
}

impl<'a, T: FromValue<'a>> FromValue<'a> for Vec<T> {
    default fn from_value(value: &'a Value) -> Option<Vec<T>> {
        match *value {
            Value::Array(ref items) => items.iter().map(FromValue::from_value).collect(),
            _ => None,
        }
    }
}

impl<'a, K: FromValue<'a>, V: FromValue<'a>> FromValue<'a> for Vec<(K, V)> {
    fn from_value(value: &'a Value) -> Option<Vec<(K, V)>> {
        let entries = match *value {
            Value::Map(ref entries) => entries,
            _ => return None,
        };

        entries.iter()
               .map(|&(ref key, ref value)| match (K::from_value(key), V::from_value(value)) {
                    (Some(key), Some(value)) => Some((key, value)),
                    _ => None,
               }).collect()
    }
}

impl<'a, K: FromValue<'a> + ::std::hash::Hash + Eq, V: FromValue<'a>> FromValue<'a> for HashMap<K, V> {
    fn from_value(value: &'a Value) -> Option<HashMap<K, V>> {
        use std::iter::FromIterator;
        Vec::<(K, V)>::from_value(value).map(FromIterator::from_iter)
    }
}

impl<'a> FromValue<'a> for DateTime<UTC> {
    fn from_value(value: &'a Value) -> Option<DateTime<UTC>> {
        String::from_value(value)
            .and_then(|s| s.parse().ok())
    }
}

impl<'a> FromValue<'a> for Date<UTC> {
    fn from_value(value: &'a Value) -> Option<Date<UTC>> {
        DateTime::from_value(value)
            .map(|dt| dt.date())
    }
}

impl<'a, T: FromValue<'a>> FromValue<'a> for Option<T> {
    fn from_value(value: &'a Value) -> Option<Option<T>> {
        match *value {
            Value::Nil => Some(None),
            _ => T::from_value(value).map(Some)
        }
    }
}

impl<'a> FromValue<'a> for &'a Value {
    #[inline]
    fn from_value(value: &'a Value) -> Option<&'a Value> {
        Some(value)
    }
}

impl<'a> FromValue<'a> for Value {
    #[inline]
    fn from_value(value: &'a Value) -> Option<Value> {
        Some(value.clone())
    }
}
