pub mod join;

message!(Message:
    type "chat:join-channel" => JoinChannel(self::join::JoinChannel),
);
