use super::conn::Connection;
use db::models::wild_monster::WildMonster;
use combat::prepare::prepare_wild_combat;
use net::msg::combat::Available;
use net::msg::combat::available::Origin;
use net::msg::MessageHeader;
use net::msg::Message;
use net::msg::result::ResultData;
use net::msg::utils::fields;
use mioco;
use std;
use rand::distributions::{IndependentSample, Range};
use rand;

pub fn send_random_combat(conn: &mut Connection) {
    let mut rng = rand::thread_rng();
    let between = Range::new(0, 140);

    loop {
        let time =std::time::Duration::from_millis(120 + between.ind_sample(&mut rng));
        mioco::sleep(time);

        let mut db_conn = conn.ctx.db.get_connection().unwrap();

        let wild_monster = match WildMonster::generate(&mut db_conn) {
            Ok(Some(monster)) => monster,
            _ => return,
        };

        let msg = Available {
            header: MessageHeader::new(),
            origin: Origin::WildMonster {
                monster_id: wild_monster.monster.id as u32
            },
            monsters_max_count: 3,
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
          ResultData::Success(ref _data) => {
              let result: bool = fields::get(_data, "accept").unwrap_or(false);
              if result {
                  let monsters: Vec<i32> = fields::get(_data, "monsters").unwrap_or(vec![]);
                  if monsters.len() > 0 {
                      prepare_wild_combat(conn, &monsters, wild_monster.monster.id);
                  }
              }
          }
          ResultData::Error { .. } => {
              continue;
          }
        }
    }
}
