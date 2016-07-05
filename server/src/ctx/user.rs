use mioco::tcp::TcpStream;
use std::collections::HashMap;

pub type UserStore = HashMap<u32, TcpStream>;
