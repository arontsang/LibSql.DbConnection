use interoptopus::{ffi, ffi_service, ffi_service_method, ffi_type};
use interoptopus::pattern::result::result_to_ffi;
use libsql::{Database};
use crate::patterns::result::Error;


#[ffi_type]
pub struct DatabaseBox {
    pub database: usize,    
}

#[ffi_type(opaque)]
pub struct DatabaseProxy {
    pub database: Box<Database>,
}


#[ffi_service]
impl DatabaseProxy {

    
    pub fn new(database: DatabaseBox) -> ffi::Result<Self, Error> {
        result_to_ffi(||{
            let database = database.database as * mut Database;
            let database = unsafe { Box::from_raw(database) };
            Ok(Self { database })
        })
    }

}

impl From<Database> for DatabaseBox {
    fn from(database: Database) -> Self {
        let database = Box::into_raw(Box::new(database));
        let database = database as usize;
        Self { database }
    }
}