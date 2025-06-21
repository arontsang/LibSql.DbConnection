use std::future::Future;
use std::sync::Arc;
use crate::patterns::result::Error;
use interoptopus::{ffi_function, ffi_service_method, ffi_type};
use interoptopus::{ffi, ffi_service};
use interoptopus::pattern::asynk::{AsyncRuntime, AsyncSelf, AsyncThreadLocal};
use interoptopus::pattern::result::{result_to_ffi, result_to_ffi_async};
use libsql::{Builder, Database};
use tokio::runtime::Runtime;
use crate::db::{DatabaseBox, DatabaseProxy};

type This = AsyncSelf<LibTursoRuntime>;
#[ffi_type(opaque)]
pub struct LibTursoRuntime {
    pub runtime: Arc<Runtime>,
}

impl AsyncRuntime for LibTursoRuntime {
    fn spawn<Fn, F>(&self, f: Fn)
    where
        Fn: FnOnce(()) -> F,
        F: Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(f(()));
    }
}

#[ffi_service]
impl LibTursoRuntime {
    pub fn new() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            // This is a workaround for the fact that tokio::runtime::Builder::new_multi_thread()
            // cannot be used in a const context.
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .map_err(|_| Error::Fail)?;

            let runtime = Arc::new(runtime);
            Ok(Self { runtime })
        })
    }

    pub async fn build_in_memory(_this: This) -> ffi::Result<DatabaseBox, Error> {
        result_to_ffi_async(async || {
            let db = Builder::new_local(":memory:").build().await
                .map_err(|_| Error::Fail)?;
            
            let db = crate::db::DatabaseBox::from(db);
            Ok(db)
        }).await
    }


}
#[ffi_function]
pub fn pattern_vec_1() -> ffi::Vec<u8> {
    vec![1, 2, 3].into()
}

