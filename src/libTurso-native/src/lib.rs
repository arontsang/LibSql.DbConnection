mod db;
mod runtime;
mod patterns;

use interoptopus::inventory::{Inventory, InventoryBuilder};
use interoptopus::{builtins_vec, extra_type, ffi, function, pattern};
use libsql::Database;

pub fn ffi_inventory() -> Inventory {
    Inventory::builder()
        .register(function!(runtime::pattern_vec_1))
        .register(builtins_vec!(u8))
        .register(pattern!(db::DatabaseProxy))
        .register(pattern!(runtime::LibTursoRuntime))
        .validate()
        .build()
}

#[unsafe(no_mangle)]
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




#[unsafe(no_mangle)]
pub extern "C" fn create_context() -> *mut Context {
    let ctx = Box::new(Context { foo: true, bar: 0, baz: 0 });
    Box::into_raw(ctx)
}

#[unsafe(no_mangle)]
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


