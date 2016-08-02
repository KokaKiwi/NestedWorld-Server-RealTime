use net::conn::Connection;
use std::collections::HashMap;

pub type UserStore = HashMap<u32, Connection>;
