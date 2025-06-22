mod db;
mod runtime;
mod patterns;
mod connection;
mod rows;

use interoptopus::inventory::{Inventory, InventoryBuilder};
use interoptopus::{builtins_vec, extra_type, ffi, function, pattern};
use libsql::Database;

pub fn ffi_inventory() -> Inventory {
    Inventory::builder()
        .register(function!(rows::rows_proxy_next))
        .register(pattern!(db::DatabaseProxy))
        .register(pattern!(connection::ConnectionProxy))
        .register(pattern!(rows::RowsProxy))
        .register(pattern!(runtime::LibTursoRuntime))
        .validate()
        .build()
}



