use super::{MessagePart, MessageHeader};
use super::error::{Result, Error};
use super::utils::fields;
use super::utils::rmp::ValueExt;
use rmp::Value;

// We use the `Message`-suffixed name here to avoid possible conflict with Rust's standard `Result`
// type.
#[derive(Debug, Clone, PartialEq)]
pub struct ResultMessage {
    pub header: MessageHeader,
    pub data: ResultData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResultData {
    Success(Value),
    Error {
        kind: String,
        message: String,
        data: Value,
    },
}

impl ResultData {
    pub fn ok(data: Option<Value>) -> ResultData {
        let data = data.unwrap_or_else(|| rmp_map![]);
        ResultData::Success(data)
    }

    pub fn err<K: Into<String>, M: Into<String>>(kind: K, message: M, data: Option<Value>) -> ResultData {
        ResultData::Error {
            kind: kind.into(),
            message: message.into(),
            data: data.unwrap_or_else(|| rmp_map![]),
        }
    }
}

impl MessagePart for ResultMessage {
    fn decode(data: &Value) -> Result<ResultMessage> {
        let header = try!(MessageHeader::decode(data));

        let result: &str = try!(fields::get(data, "result"));
        let mut data = data.clone();
        data.remove("id");
        data.remove("type");
        data.remove("result");

        let data = match result {
            "success" => {
                ResultData::Success(data)
            }
            "error" => {
                let kind = try!(fields::get(&data, "kind"));
                let message = try!(fields::get(&data, "message"));

                data.remove("kind");
                data.remove("message");

                ResultData::Error {
                    kind: kind,
                    message: message,
                    data: data,
                }
            }
            _ => return Err(Error::InvalidField("result", format!("Bad result type `{}`, should be `success` or `error`", result))),
        };

        Ok(ResultMessage {
            header: header,
            data: data,
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "result");
        self.header.encode(data);
        match self.data {
            ResultData::Success(ref r_data) => {
                data.set("result", "success");
                data.extend(r_data);
            }
            ResultData::Error {
                ref kind,
                ref message,
                data: ref r_data,
            } => {
                data.set("result", "error");
                data.set("kind", kind);
                data.set("message", message);
                data.extend(r_data);
            }
        }
    }
}
