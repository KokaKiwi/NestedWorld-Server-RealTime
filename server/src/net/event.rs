use super::conn::Connection;
use net::msg::combat::Available;
use net::msg::combat::available::Origin;
use net::msg::MessageHeader;
use net::msg::Message;
use net::msg::result::ResultData;
use mioco;
use std;
use rand::distributions::{IndependentSample, Range};
use rand;
use uuid::Uuid;

pub fn send_random_combat(conn: &mut Connection) {
    let mut rng = rand::thread_rng();
    let between = Range::new(0, 140);

    loop {
        let time =std::time::Duration::from_millis(120 + between.ind_sample(&mut rng));
        mioco::sleep(time);

        let msg = Available {
            header: MessageHeader {
                id: Some(Uuid::new_v4().to_string()),
            },
            origin: Origin::WildMonster {
                monster_id: 1,
            },
        };

        let rx = match conn.send_request(msg) {
          Ok(rx) => rx,
          Err(e) => {
            debug!("{}", e);
            continue;
          }
        };

        let msg = match rx.recv() {
          Ok(Message::Result(msg)) => msg,
          _ => {
            debug!("Inappropirate type of packet",);
            continue;
          }
        };

        match msg.data {
          ResultData::Success(ref data) => {
              //start_pve_combat(conn, data);
          },
          ResultData::Error { .. } => {
              continue;
          }
        }
    }
}
