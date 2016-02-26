//! This module contains all the needed elements to communicate with the server thread(s)
use chan;
use self::cmd::Command;

pub mod cmd;
pub mod res;

/// A server handle used to send and receive messages from the server thread(s).
#[derive(Debug, Clone)]
pub struct ServerHandle {
    sender: chan::Sender<Command>,
}
