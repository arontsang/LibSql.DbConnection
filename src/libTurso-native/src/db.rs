use interoptopus::{ffi_service_method, ffi_type};
use libsql::Database;

#[ffi_type(opaque)]
#[allow(dead_code)]
pub struct LibTursoDatabase {
    pub(crate) db: Box<Database>
}

impl LibTursoDatabase {
    #[ffi_service_method(ignore)]
    pub fn new(db: Box<Database>) -> LibTursoDatabase {
        Self { db }
    }

    pub async fn get_one() -> u8 {
        1
    }
}