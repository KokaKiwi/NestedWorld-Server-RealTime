use rmp::Value;
use net::msg::MessagePart;
use net::msg::error::{Result, Error};
use super::rmp::ValueExt;

pub trait FieldType: Sized {
    fn get(data: &Value, key: &'static str) -> Result<Self>;
    fn set(&self, data: &mut Value, key: &'static str);
}

impl FieldType for String {
    fn get(data: &Value, key: &'static str) -> Result<String> {
        data.get_str(key).map(String::from).ok_or(Error::MissingField(key))
    }

    fn set(&self, data: &mut Value, key: &'static str) {
        data.set(key, self);
    }
}

impl<M: MessagePart> FieldType for M {
    fn get(data: &Value, key: &'static str) -> Result<M> {
        data.get(key).ok_or(Error::MissingField(key))
            .and_then(M::decode)
    }

    fn set(&self, data: &mut Value, key: &'static str) {
        let mut value = rmp_map![];
        self.encode(&mut value);
        data.set(key, value);
    }
}

impl<T: FieldType> FieldType for Option<T> {
    fn get(data: &Value, key: &'static str) -> Result<Option<T>> {
        Ok(T::get(data, key).ok())
    }

    fn set(&self, data: &mut Value, key: &'static str) {
        if let Some(ref field) = *self {
            field.set(data, key)
        }
    }
}
