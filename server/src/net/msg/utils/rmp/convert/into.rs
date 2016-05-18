use rmp::value::{Value, Integer, Float};
use std::collections::HashMap;

pub trait IntoValue {
    fn into_value(self) -> Value;
}

pub trait ToValue {
    fn to_value(&self) -> Value;
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Boolean(self)
    }
}

macro_rules! int_convert {
    ($ctor:ident, $ty:ty) => {
        impl IntoValue for $ty {
            fn into_value(self) -> Value {
                Value::Integer(Integer::$ctor(self as _))
            }
        }
    };
}

int_convert!(I64, i8);
int_convert!(I64, i16);
int_convert!(I64, i32);
int_convert!(I64, i64);
int_convert!(U64, u8);
int_convert!(U64, u16);
int_convert!(U64, u32);
int_convert!(U64, u64);

impl IntoValue for f32 {
    fn into_value(self) -> Value {
        Value::Float(Float::F32(self))
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Float(Float::F64(self))
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoValue for Vec<u8> {
    fn into_value(self) -> Value {
        Value::Binary(self)
    }
}

impl<'a, T: ToValue + ?Sized> IntoValue for &'a T {
    fn into_value(self) -> Value {
        self.to_value()
    }
}

impl<T: IntoValue> IntoValue for Vec<T> {
    default fn into_value(self) -> Value {
        Value::Array(self.into_iter().map(IntoValue::into_value).collect())
    }
}

impl<K: IntoValue, V: IntoValue> IntoValue for Vec<(K, V)> {
    fn into_value(self) -> Value {
        Value::Map(self.into_iter()
                       .map(|(key, value)| (key.into_value(), value.into_value()))
                       .collect())
     }
}

impl<K: IntoValue + ::std::hash::Hash + Eq, V: IntoValue> IntoValue for HashMap<K, V> {
    fn into_value(self) -> Value {
        Value::Map(self.into_iter()
                       .map(|(key, value)| (key.into_value(), value.into_value()))
                       .collect())
    }
}

// Implement `IntoValue` for optionnable types, with `nil` for `None` value.
impl<T: IntoValue> IntoValue for Option<T> {
    fn into_value(self) -> Value {
        match self {
            Some(value) => value.into_value(),
            None => Value::Nil,
        }
    }
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl ToValue for str {
    fn to_value(&self) -> Value {
        self.to_owned().into_value()
    }
}

impl<T: ToValue> ToValue for [T] {
    default fn to_value(&self) -> Value {
        self.iter().map(ToValue::to_value).collect::<Vec<_>>().into_value()
    }
}

impl ToValue for [u8] {
    fn to_value(&self) -> Value {
        self.to_owned().into_value()
    }
}

impl<T: Clone + IntoValue> ToValue for T {
    default fn to_value(&self) -> Value {
        self.clone().into_value()
    }
}
