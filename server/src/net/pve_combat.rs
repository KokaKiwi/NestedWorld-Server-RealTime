use combat::Combat;
use combat::CombatStore;
use net::conn::Connection;
use net::msg::result::ResultData;

fn start_pve_combat(conn: &mut Connection, data: ResultData) {
    let mut combat = conn.ctx.combats.create();

    combat.state.start();
}
