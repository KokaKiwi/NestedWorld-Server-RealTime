use rmp::Value;
use net::msg::error::{Result, ErrorKind};
use net::msg::utils::rmp::FromValue;
use super::rmp::ValueExt;

pub fn get<'a, T: FromValue<'a>>(data: &'a Value, key: &'static str) -> Result<T> {
    data.get::<_, &'a Value>(key).ok_or(ErrorKind::MissingField(key).into())
        .and_then(|value| value.to().ok_or(ErrorKind::InvalidField(key, format!("Invalid field type")).into()))
}
