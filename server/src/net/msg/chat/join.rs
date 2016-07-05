use net::msg::{MessagePart, MessageFull, MessageHeader};
use net::msg::error::Result;
use net::msg::utils::fields;
use net::msg::utils::rmp::ValueExt;
use rmp::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct JoinChannel {
    pub header: MessageHeader,
    pub channel: String,
}

impl MessagePart for JoinChannel {
    fn decode(data: &Value) -> Result<JoinChannel> {
        Ok(JoinChannel {
            header: try!(MessageHeader::decode(data)),
            channel: try!(fields::get(data, "channel")),
        })
    }

    fn encode(&self, data: &mut Value) {
        data.set("type", "chat:join-channel");
        self.header.encode(data);
        data.set("channel", &self.channel);
    }
}

impl MessageFull for JoinChannel {
    fn header(&self) -> &MessageHeader {
        &self.header
    }

    fn header_mut(&mut self) -> &mut MessageHeader {
        &mut self.header
    }
}
