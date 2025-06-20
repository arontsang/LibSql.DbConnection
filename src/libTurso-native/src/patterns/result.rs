use interoptopus::{ffi_function, ffi_type};

#[ffi_type]
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Error {
    Fail,
}