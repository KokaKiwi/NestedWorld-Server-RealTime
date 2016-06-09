pub use self::join::JoinChannel;

pub mod join;

message!(Message {
    type "chat:join-channel" => JoinChannel(JoinChannel),
});
