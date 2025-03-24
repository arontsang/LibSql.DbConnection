use libsql::Builder;

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
// 
// #[no_mangle]
// pub extern "C" fn create_list(
//     mut list: * mut Context,
//     length: &mut usize) -> () {
//     let ret = Box::new([Context { foo: true, bar: 0, baz: 0 }, Context { foo: true, bar: 0, baz: 0 }]);
//     *length = 2;
//     //*list = ret;
//     list = Box::into_raw(ret);
//     ()
// }



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


