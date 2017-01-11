use ctx::Context;
use db::models::token::Session;
use mioco;
use mioco::sync::mpsc as chan;
use mioco::sync::Mutex;
use mioco::tcp::TcpStream;
use super::msg::{Message, MessagePart, MessageFull};
use rmp::decode::value::read_value;
use rmp::encode::value::Error as EncodeError;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use super::handlers;
use super::event;
use net::msg::MessageHeader;
use net::msg::result::ResultData;
use net::handlers::helpers::result::send_result;

pub struct Connection {
    open: Arc<AtomicBool>,
    pub ctx: Context,
    pub stream: TcpStream,
    pub session: Arc<Mutex<Option<Session>>>,
    conversations: Arc<Mutex<HashMap<String, chan::Sender<Message>>>>,
}

impl Connection {
    pub fn new(ctx: Context, stream: TcpStream) -> Connection {
        Connection {
            open: Arc::new(AtomicBool::new(true)),
            ctx: ctx,
            stream: stream,
            session: Arc::new(Mutex::new(None)),
            conversations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn send<M: MessagePart + ::std::fmt::Debug>(&mut self, msg: M) -> Result<(), EncodeError> {
        use rmp::encode::value::write_value;
        debug!("[{}] <- {:?}", self.name(), msg);
        write_value(&mut self.stream, &msg.value())
    }

    pub fn send_request<M: MessageFull + ::std::fmt::Debug>(&mut self, mut msg: M) -> Result<chan::Receiver<Message>, EncodeError> {
        let id = msg.header_mut().ensure_id();

        self.send(msg).map(|_| {
            self.register_request(id)
        })
    }

    pub fn register_request(&self, id: String) -> chan::Receiver<Message> {
        let (tx, rx) = chan::channel();
        let mut conversations = self.conversations.lock().unwrap_or_else(|e| e.into_inner());
        conversations.insert(id, tx);
        rx
    }

    pub fn get_conversation(&mut self, id: &str) -> Option<chan::Sender<Message>> {
        let mut conversations = self.conversations.lock().unwrap_or_else(|e| e.into_inner());
        conversations.remove(id)
    }

    pub fn name(&self) -> String {
        let peer_addr = self.stream.peer_addr().unwrap();
        match self.session() {
            Some(session) => {
                let ref pseudo = session.user.get().unwrap().pseudo;
                format!("{} ({})", pseudo, peer_addr)
            }
            None => peer_addr.to_string(),
        }
    }

    pub fn session(&self) -> Option<Session> {
        mutex_lock!(self.session).clone()
    }

    pub fn open(&self) -> bool {
        use std::sync::atomic::Ordering;
        self.open.load(Ordering::SeqCst)
    }

    pub fn close(&self) {
        use std::sync::atomic::Ordering;
        self.open.store(false, Ordering::SeqCst);
    }

    pub fn try_clone(&self) -> ::std::io::Result<Connection> {
        Ok(Connection {
            open: self.open.clone(),
            ctx: self.ctx.clone(),
            stream: try!(self.stream.try_clone()),
            session: self.session.clone(),
            conversations: self.conversations.clone(),
        })
    }
}

impl ::std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.name().fmt(f)
    }
}

pub fn run(ctx: Context, conn: TcpStream) {
    let conn = Connection::new(ctx, conn);

    debug!("Got connection!");

    let read_handle = match conn.try_clone() {
        Ok(mut conn) => {
            mioco::spawn(move || read_and_decode(&mut conn))
        }
        Err(err) => {
            debug!("Error when trying to clone TcpStream connection : {}", err);
            return;
        }
    };

    let _event_handle = match conn.try_clone() {
        Ok(mut conn) => {
            debug("Start this fucking THREAD !")
            mioco::spawn(move || event::send_random_combat(&mut conn))
        }
        Err(err) => {
            debug!("Error when trying to clone TcpStream connection : {}", err);
            return;
        }
    };

    let _ = read_handle.join();

    match conn.session() {
        Some(mut session) => {
            let db_conn = conn.ctx.db.get_connection().unwrap();
            let ref user = session.user.get_or_fetch(&db_conn).unwrap().expect("No user?");
            let mut users = conn.ctx.users.lock().unwrap_or_else(|e| e.into_inner());
            users.remove(&(user.id as u32));
        }
        None => {}
    }
    conn.close();
}

pub fn read_and_decode(conn: &mut Connection) {
    loop {
        let msg = match read_value(&mut conn.stream) {
            Ok(msg) => msg,
            Err(e) => {
                send_result(conn, &MessageHeader::new(), ResultData::err("internal", e.description(), None));
                debug!("[{}] Error reading MessagePack value: {}", conn.name(), e);
                break;
            }
        };
        debug!("[{}] -RAW-> {:?}", conn.name(), msg);

        let msg = match Message::decode(&msg) {
            Ok(msg) => msg,
            Err(e) => {
                debug!("[{}] Received an invalid message: {}", conn.name(), e);
                send_result(conn, &MessageHeader::new(), ResultData::err("InvalidMsg", e.description(), None));
                continue;
            }
        };
        debug!("[{}] -> {:?}", conn.name(), msg);

        handlers::handle(conn, msg);
    }
}
