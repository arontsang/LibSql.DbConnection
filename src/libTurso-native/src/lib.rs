mod db;
mod runtime;
mod patterns;

use interoptopus::inventory::{Inventory, InventoryBuilder};
use interoptopus::{extra_type, function, pattern};


pub fn ffi_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(extra_type!(db::LibTursoDatabase))
        .register(pattern!(runtime::LibTursoRuntime))
        // .register(pattern!(asynk::AsyncService))
        // .register(function!(scheduler::poll_scheduler))
        .validate()
        .build()
}

#[no_mangle]
pub extern "C" fn my_add(x: i32, y: i32) -> i32 {
    println!("hello there!");
    x + y
}

#[repr(C)]
pub struct Context {
    pub foo: bool,
    pub bar: i32,
    baz: u64
}




#[no_mangle]
pub extern "C" fn create_context() -> *mut Context {
    let ctx = Box::new(Context { foo: true, bar: 0, baz: 0 });
    Box::into_raw(ctx)
}

#[no_mangle]
pub extern "C" fn delete_context(context: *mut Context) {
    let _ = unsafe {
        Box::from_raw(context)
    };
}


impl Drop for Context {
    fn drop(&mut self) {
        println!("Dropped!")
    }
}


