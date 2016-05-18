use rmp::Value;
use net::msg::error::{Result, Error};
use net::msg::utils::rmp::FromValue;
use super::rmp::ValueExt;

pub fn get<'a, T: FromValue<'a>>(data: &'a Value, key: &'static str) -> Result<T> {
    data.get::<_, &'a Value>(key).ok_or(Error::MissingField(key))
        .and_then(|value| value.to().ok_or(Error::InvalidField(key, format!("Invalid field type"))))
}
