use chan::Sender;
use super::res::Response;

#[derive(Debug, Clone)]
pub struct Command {
    pub data: CommandData,
    pub sender: Sender<Response>,
}

#[derive(Debug, Clone)]
pub enum CommandData {}
