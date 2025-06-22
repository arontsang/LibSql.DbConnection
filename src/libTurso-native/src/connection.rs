use std::sync::Arc;
use interoptopus::{ffi, ffi_service, ffi_type};
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncSelf};
use interoptopus::pattern::result::{result_to_ffi, result_to_ffi_async};
use libsql::{Connection, Database};
use tokio::runtime::Runtime;
use crate::db::{DatabaseBox, DatabaseProxy};
use crate::patterns::result::Error;
use crate::rows::RowsBox;

#[ffi_type]
pub struct ConnectionBox {
    pub connection: usize,
    pub runtime: usize,
}

#[ffi_type(opaque)]
pub struct ConnectionProxy {
    connection: Box<Connection>,
    runtime: Arc<Box<Runtime>>,
}
type This = AsyncSelf<ConnectionProxy>;
#[ffi_service]
impl ConnectionProxy {
    pub fn new(database: ConnectionBox) -> ffi::Result<Self, Error> {
        result_to_ffi(||{
            let runtime = database.runtime as * mut Box<Runtime>;
            let runtime = unsafe { Arc::from_raw(runtime) };

            let connection = database.connection as * mut Connection;
            let connection = unsafe { Box::from_raw(connection) };

            Ok(Self { connection, runtime })
        })
    }

    pub async fn query(this: This, sql: ffi::String) -> ffi::Result<RowsBox, Error> {
        result_to_ffi_async(async || {
            let rows = this.connection.query(sql.as_str(), ()).await
                .map_err(|_| Error::Fail)?;


            let runtime = this.runtime.clone();
            let runtime = Arc::into_raw(runtime) as usize;


            let rows = Box::new(rows);
            let rows = Box::into_raw(rows) as usize;
            
            Ok(RowsBox{ rows, runtime })
        }).await
    }
}

impl AsyncRuntime for ConnectionProxy {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}