pub use self::foreign::Relation;

mod foreign;

pub trait Model: Sized {
    fn get_by_id(conn: &::postgres::Connection, id: i32) -> ::postgres::Result<Option<Self>>;
}
