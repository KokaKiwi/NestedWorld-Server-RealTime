use super::conn::Connection;
use net::msg::combat::Available;
use net::msg::combat::available::Origin;
use net::msg::MessageHeader;
use mioco;
use rand::distributions::{IndependentSample, Range};
use rand;
use uuid::Uuid;

pub fn send_random_combat(conn: &mut Connection) {
    let mut rng = rand::thread_rng();
    let between = Range::new(0, 140);

    loop {
        let time = 120 + between.ind_sample(&mut rng) * 1000;
        mioco::sleep_ms(time);

        let msg = Available {
            header: MessageHeader {
                id: Some(Uuid::new_v4().to_string()),
            },
            origin: Origin::WildMonster {
                monster_id: 1,
            },
        };

        match conn.send(msg) {
            Ok(_) => {}
            Err(e) => {
                debug!("Error when sending result: {}", e);
            }
        }
    }
}
