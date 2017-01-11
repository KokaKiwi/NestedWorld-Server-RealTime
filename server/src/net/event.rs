use super::conn::Connection;
use db::models::wild_monster::WildMonster;
use db::models::monster::Monster;
use db::models::commons::ElementType;
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
    debug!("Starting the random combat !");
    let mut rng = rand::thread_rng();
    let between = Range::new(0, 120);

    loop {
        let time =std::time::Duration::from_millis(140 + between.ind_sample(&mut rng));
        mioco::sleep(time);

        // let db_conn = conn.ctx.db.get_connection().unwrap();

        debug!("Lets create the monster");
        let wild_monster = WildMonster {
            monster: Monster {
                id: 1,
                name: String::from("hateful table"),
                monster_type: ElementType::Earth,
                attack: 38.0,
                hp: 32.0,
                speed: 47.0,
                defense: 71.0,
            }
        };

        debug!("monster created");
        let mut header = MessageHeader::new();
        let uuid = header.ensure_id();
        let msg = Available {
            header: header.clone(),
            origin: Origin::WildMonster {
                monster_id: wild_monster.monster.id as u32
            },
            monsters_max_count: 3,
        };

        let rx = match conn.send_request(msg) {
          Ok(rx) => rx,
          Err(e) => {
            debug!("Wild monster : {}", e);
            continue;
          }
        };
        debug!("sended request !");
        let msg = match rx.recv() {
          Ok(Message::Result(msg)) => msg,
          _ => {
            debug!("Inappropirate type of packet",);
            continue;
          }
        };
        debug!("recieve respond !");
        match msg.data {
          ResultData::Success(ref _data) => {
              let result: bool = fields::get(_data, "accept").unwrap_or(false);
              if result {
                  let monsters: Vec<i32> = fields::get(_data, "monsters").unwrap_or(vec![]);
                  if monsters.len() > 0 {
                      debug!("GO TO THE COMBAT !");
                      prepare_wild_combat(conn, &monsters, wild_monster.monster.id, uuid);
                  }
              }
          }
          ResultData::Error { .. } => {
              continue;
          }
        }
    }
}
