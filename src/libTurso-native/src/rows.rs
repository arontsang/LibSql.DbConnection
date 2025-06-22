
use std::sync::Arc;
use interoptopus::{ffi, ffi_function, ffi_service, ffi_type};
use interoptopus::pattern::result::result_to_ffi;
use libsql::{Row, Rows};
use tokio::runtime::Runtime;
use tokio::sync::RwLock;
use crate::patterns::result::Error;

#[ffi_type]
pub struct RowsBox {
    pub rows: usize,
    pub runtime: usize,
}


#[ffi_type(opaque)]
pub struct RowsProxy {
    rows: Box<Rows>,
    runtime: Arc<Box<Runtime>>,
    current_row: Arc<RwLock<Option<Row>>>,
}

#[ffi_function]
pub unsafe fn rows_proxy_next(rows: *mut RowsProxy, callback: extern "C" fn(usize, usize) -> (), state: usize) -> u8 {

    let runtime = &(unsafe { &mut *rows }.runtime);
    let rows = unsafe { &mut *rows };

    runtime.spawn(async move {
        let result = rows.rows.next().await;
        let current_row = rows.current_row.clone();
        let current_row = current_row.as_ref();
        match result {
            Ok(row) => {
                let mut write = current_row.write().await;
                if let Some(row) = row {
                    write.replace(row);
                    callback(1, state)
                } else {
                    write.take();
                    callback(0, state)
                }
            }
            Err(_) => {
                callback(3, state)
            }
        }
    });
    3
}

#[ffi_service]
impl RowsProxy {
    pub fn new(database: RowsBox) -> ffi::Result<Self, Error> {
        result_to_ffi(||{
            let runtime = database.runtime as * mut Box<Runtime>;
            let runtime = unsafe { Arc::from_raw(runtime) };

            let rows = database.rows as * mut Rows;
            let rows = unsafe { Box::from_raw(rows) };

            Ok(Self { rows, runtime, current_row: Arc::new(RwLock::new(None)) })
        })
    }
}