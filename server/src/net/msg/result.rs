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

impl MessagePart for ResultMessage {
    fn decode(data: &Value) -> Result<ResultMessage> {
        let header = try!(MessageHeader::decode(data));

        let result: &str = try!(fields::get(data, "result"));
        let mut data = data.clone();
        data.remove("id");
        data.remove("type");
        data.remove("result");

        match result {
            "success" => {
                unimplemented!()
            }
            "error" => {
                unimplemented!()
            }
            _ => Err(Error::InvalidField("result", format!("Bad result type `{}`, should be `success` or `error`", result))),
        }
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "result");
        self.header.encode(data);
        match self.data {
            ResultData::Success(ref data) => {}
            ResultData::Error {
                kind: ref kind,
                message: ref message,
                data: ref data,
            } => {}
        }
    }
}
