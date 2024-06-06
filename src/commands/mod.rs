use crate::database::Db;

pub mod add;
pub mod print;
pub mod sync;

pub struct CmdCtx<T> {
    pub db: Db,
    pub args: T,
}

impl<T> CmdCtx<T> {
    pub fn new(db: Db, args: T) -> Self {
        Self { db, args }
    }
}
