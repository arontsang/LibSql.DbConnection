use std::sync::Arc;
use interoptopus::{ffi, ffi_service, ffi_service_method, ffi_type};
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncSelf};
use interoptopus::pattern::result::{result_to_ffi, result_to_ffi_async};
use libsql::{Database};
use tokio::runtime::Runtime;
use crate::connection::ConnectionBox;
use crate::patterns::result::Error;


type This = AsyncSelf<DatabaseProxy>;

#[ffi_type]
pub struct DatabaseBox {
    pub database: usize,
    pub runtime: usize, 
}

impl DatabaseBox {
    pub(crate) fn new(database: Database, runtime: Arc<Box<Runtime>>) -> Self {
        let database = Box::into_raw(Box::new(database)) as usize;

        let runtime = Arc::into_raw(runtime) as usize;
        Self { database, runtime }
    }
}


#[ffi_type(opaque)]
pub struct DatabaseProxy {
    database: Box<Database>,
    runtime: Arc<Box<Runtime>>,
}


#[ffi_service]
impl DatabaseProxy {
   
    pub fn new(database: DatabaseBox) -> ffi::Result<Self, Error> {
        result_to_ffi(||{
            let runtime = database.runtime as * mut Box<Runtime>;
            let runtime = unsafe { Arc::from_raw(runtime) };                
                
            let database = database.database as * mut Database;
            let database = unsafe { Box::from_raw(database) };            
            
            Ok(Self { database, runtime })
        })
    }
    
    pub fn create_connection(&self) -> ffi::Result<ConnectionBox, Error> {
        result_to_ffi(|| {
            let connection = self.database.connect()
                .map_err(|_| Error::Fail)?;
            
            let connection = Box::new(connection);
            let connection = Box::into_raw(connection) as usize;
            let runtime = self.runtime.clone();
            let runtime = Arc::into_raw(runtime) as usize;

            Ok(ConnectionBox { connection, runtime })
        })
    }
}

impl AsyncRuntime for DatabaseProxy {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}