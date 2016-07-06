use net::conn::Connection;
use net::msg::Authenticate;
use net::msg::result::ResultData;
use super::helpers::result::handle_with_result;

pub fn handle(conn: &mut Connection, msg: Authenticate) {
    handle_with_result(conn, &msg.header, |conn| {
        let session = match msg.session(&conn.ctx.config.secret) {
            Ok(session) => session,
            Err(e) => {
                debug!("Invalid token: {}: {:?}", msg.token, e);
                return ResultData::err("InvalidToken", "Invalid token provided.", None)
            }
        };
        let session = match session.db(&conn.ctx.db) {
            Ok(Some(session)) => session,
            Ok(None) => return ResultData::err("InvalidToken", "Invalid token provided.", None),
            Err(e) => {
                debug!("Database error: {}", e);
                return ResultData::err("InternalError", "Internal server error.", None);
            }
        };

        {
            let ref user = session.user.get().unwrap();
            let mut users = conn.ctx.users.lock().unwrap_or_else(|e| e.into_inner());

            if let Ok(conn) = conn.try_clone() {
                users.insert(user.id as u32, conn);
            }
        }

        conn.session = Some(session);
        ResultData::ok(None)
    });
}
