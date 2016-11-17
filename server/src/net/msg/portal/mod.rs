pub use self::capture::Capture;

pub mod capture;

message!(Message {
    type "portal:capture" => Capture(Capture),
});
